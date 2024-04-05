use crate::factory::{
    shell::exec_once,
    shell::spawn_once,
};

use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;

pub fn bat_label(name: &str) -> gtk::Label {
    let label = gtk::Label::new(None);
    let label_clone = label.clone();

    label.set_widget_name(name);

    let callback = move || {
        let text = exec_once("cat /sys/class/power_supply/BAT0/capacity");
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
