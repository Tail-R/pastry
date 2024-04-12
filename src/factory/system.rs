use crate::factory::{
    shell::{
        exec_once,
        spawn_once,
        spawn_listen,
    },
    misc::{
        label,
    },
    scripts::{
        GET_BAT_CAP,
        GET_BR,
        FOLLOW_BR,
        GET_VOL,
        FOLLOW_VOL,
        FOLLOW_NM_STATE,
    },
};

use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;

use networkmanager::{
    NetworkManager,
    devices::{
        Device,
        Wireless,
    },
};

use dbus::blocking::Connection;

pub fn bat_cap_label(name: &str) -> gtk::Label {
    let label = label(name, "");
    let label_clone = label.clone();

    let callback = move || {
        let text = exec_once(GET_BAT_CAP);
        label_clone.set_text(&text);

        if let Ok(cap) = text.parse::<i32>() {
            if cap < 20 {
                spawn_once("notify-send 'battery is low!'");
            }
        }

        glib::ControlFlow::Continue
    };

    callback(); // init

    let sec = Duration::new(60, 0);
    glib::timeout_add_local(sec, callback);

    label
}

fn shell_callback(name: &str, cmd: &str, track_cmd: &str) -> gtk::Label {
    let label = label(name, &exec_once(cmd));
    let label_clone = label.clone();
    let cmd_clone = cmd.to_string();

    let r = spawn_listen(track_cmd);

    glib::MainContext::default().spawn_local(async move {
        while let Ok(_) = r.recv().await {
            let text = exec_once(&cmd_clone);
            label_clone.set_text(&text);
        }
    });

    label
}

pub fn brightness_label(name: &str) -> gtk::Label {
    shell_callback(name, GET_BR, FOLLOW_BR)
}

pub fn volume_label(name: &str) -> gtk::Label {
    shell_callback(name, GET_VOL, FOLLOW_VOL)
}

fn get_ap_ssid() -> String {
    if let Ok(dbus_connection) = Connection::new_system() {
        let nm = NetworkManager::new(&dbus_connection);

        for dev in nm.get_devices().unwrap() {
            match dev {
                Device::WiFi(x) => {
                    if let Ok(ap) = x.active_access_point() {
                        if let Ok(ssid) = ap.ssid() {
                            return ssid;
                        }
                    }
                },
                Device::Ethernet(_) => {
                    return "Ethernet".to_string()
                },
                _ => {}
            }
        }
    }

    "Disconnected".to_string()
}

pub fn network_label(name: &str) -> gtk::Label {
    let label = label(name, &get_ap_ssid());
    let label_clone = label.clone();

    let r = spawn_listen(FOLLOW_NM_STATE);

    glib::MainContext::default().spawn_local(async move {
        while let Ok(_) = r.recv().await {
            let text = get_ap_ssid();
            label_clone.set_text(&text);
        }
    });

    label
}
