use std::thread;
use std::time::Duration;
use std::process::{
    Command,
    Stdio
};

use std::io::Read;

use async_channel;
use async_channel::Receiver;

use futures::executor::block_on;

use gtk::glib;
use gtk::prelude::*;

pub fn exec_once(command: &str) -> String {
    let mut shell = Command::new("sh");
    let cmd = shell.arg("-c").arg(command);

    let res = match cmd.output() {
        Ok(out) => String::from_utf8_lossy(&out.stdout).replace("\n", ""),
        Err(_) => "".to_string()
    };

    res
}

// It doesn't block main thread
pub fn spawn_once(command: &str) {
    let mut shell = Command::new("sh");
    let cmd = shell.arg("-c").arg(command);

    if let Err(_) = cmd.spawn() {
        println!("Failed to start command");
    }
}

pub fn spawn_listen(command: &str) -> Receiver<String> {
    let cmd = String::from(command);
    let (sender, receiver) = async_channel::unbounded();

    let process = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start process");

    if let Some(mut stdout) = process.stdout {
        thread::spawn(move || {
            block_on(async move {
                loop {
                    let mut buf = vec![0; 256];
                    let _ = stdout.read(&mut buf);
                    let text = String::from_utf8_lossy(&buf)
                        .replace("\0", "")
                        .replace("\n", "");

                    let _ = sender.send(text.to_string()).await;
                }
            });
        });
    }

    receiver
}

pub fn poll_label(name: &str, command: &str, timeout: u64) -> gtk::Label {
    let cmd = String::from(command);
    let label = gtk::Label::new(None);
    let label_clone = label.clone();

    label.set_widget_name(name);

    let callback = move || {
        let text = exec_once(&cmd);
        label_clone.set_text(&text);

        glib::ControlFlow::Continue
    };

    callback(); // init

    let sec = Duration::new(timeout, 0);
    glib::timeout_add_local(sec, callback);

    label
}

pub fn listen_label(name: &str, command: &str) -> gtk::Label {
    let cmd = String::from(command);
    let label = gtk::Label::new(None);
    let label_clone = label.clone();

    label.set_widget_name(name);

    let process = match Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn() {
        
        Ok(child) => child,
        Err(_) => {
            label.set_text("Failed to run command");
            return label;
        }
    };

    let mut stdout = match process.stdout {
        Some(stdout) => stdout,
        None => {
            label.set_text("Failed to get a handle for the child's stdout");
            return label;
        }
    };

    let (s, r) = async_channel::unbounded();

    // Sender thread    
    thread::spawn(move || {
        block_on(async move {
            loop {
                let mut buf = vec![0; 128];
                let _ = stdout.read(&mut buf);
                let text = String::from_utf8_lossy(&buf)
                    .replace("\0", "")
                    .replace("\n", "");

                let _ = s.send(text.to_string()).await;
            }
        });
    });

    // Receiver thread
    glib::MainContext::default().spawn_local(async move {
        while let Ok(text) = r.recv().await {
            label_clone.set_text(&text);
        }
    });

    label
}
