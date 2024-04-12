use crate::widgets::{
    custom_window::Window,
    custom_box::Box,
    custom_button::Button,
};

use crate::factory::{
    shell::{
        exec_once,
        spawn_once,
        spawn_listen,
    },
    misc::{
        label,
    },
};

use gtk::glib;
use gtk::prelude::*;
use gtk::Orientation::{
    Horizontal as H,
    Vertical as V,
};

use mpris::PlayerFinder;
use mpris::Event::*;

fn mpris_title_label(name: &str) -> Box {
    Box::new(name, V)
}

fn follow_metadata() -> (String, String) {
    let player = PlayerFinder::new()
        .expect("Could not connect to D-Bus")
        .find_active()
        .expect("Could not find active player");

    let mut events = player.events()
        .expect("Could not start event stream");

    while let Some(Ok(event)) = events.next() {
        match event {
            TrackChanged(metadata) => {
                println!("{:?}", metadata);
            },
            _ => {}
        }
    }

    ("a".to_string(), "b".to_string())
}
