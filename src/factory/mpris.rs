use crate::helper::get_substring;

use crate::widgets::{
    custom_box::Box,
    custom_button::Button,
};

use crate::factory::{
    shell::{
        exec_once,
        spawn_listen,
    },
    misc::{
        label,
    },
    scripts::{
        MPRIS_PREV,
        MPRIS_TOGGLE,
        MPRIS_NEXT,
        FOLLOW_MPRIS_METADATA,
    },
};

use gtk::glib;
use gtk::prelude::*;
use gtk::Align::*;
use gtk::Orientation::{
    Horizontal as H,
};

#[allow(dead_code)]
enum MprisAction {
    PREV,
    TOGGLE,
    NEXT,
}

use MprisAction::*;

const TITLE_MAX_LEN: i32 = 100;
const ARTIST_MAX_LEN: i32 = 100;

pub fn player_box(name: &str) -> Box {
    let title = label("title", "no media");
    let artist = label("artist", "unknown");

    let title_clone = title.clone();
    let artist_clone = artist.clone();

    let r = spawn_listen(FOLLOW_MPRIS_METADATA);

    // callback
    glib::MainContext::default().spawn_local(async move{
        while let Ok(metadata_dirty) = r.recv().await {
            let metadata: Vec<&str> = metadata_dirty.split("%sep%").collect();

            if metadata.len() == 2 {
                if metadata[0] != "" {
                    title_clone.set_text(&get_substring(metadata[0], TITLE_MAX_LEN));
                } else {
                    title_clone.set_text("no media");
                }

                if metadata[1] != "" {
                    artist_clone.set_text(&get_substring(metadata[1], ARTIST_MAX_LEN));
                } else {
                    artist_clone.set_text("unknown");
                }
            }
        }
    });

    Box::new(name, H).halign(Center).load(vec![
        Box::new("", H).child(label("title", "now playing ♪ ")),
        Box::new("", H).child(title),
        Box::new("", H).child(label("artist", " by ")),
        Box::new("", H).child(artist),
    ])
}

#[allow(dead_code)]
fn get_button(name: &str, action: MprisAction) -> Button {
    let button = Button::new(name);

    match action {
        PREV => button.set_label("prev"),
        TOGGLE => button.set_label("pause"),
        NEXT => button.set_label("next"),
    };

    button.connect_clicked(move |_| {
        match action {
            PREV => exec_once(MPRIS_PREV),
            TOGGLE => exec_once(MPRIS_TOGGLE),
            NEXT => exec_once(MPRIS_NEXT),
        };
    });

    button
}
