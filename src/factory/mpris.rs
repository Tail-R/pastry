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
        sep,
        pad,
        img_box_simple,
    },
    scripts::{
        MPRIS_PREV,
        MPRIS_TOGGLE,
        MPRIS_NEXT,
        FOLLOW_MPRIS_METADATA,
        GET_IMG_GEOMETRY,
    },
};

use gtk::glib;
use gtk::prelude::*;
use gtk::Align::*;
use gtk::Orientation::{
    Horizontal as H,
    Vertical as V,
};

enum MprisAction {
    PREV,
    TOGGLE,
    NEXT,
}

use MprisAction::*;

const ARTPIC_BASE: i32 = 40;
const TITLE_MAX_LEN: i32 = 25;
const ARTIST_MAX_LEN: i32 = 20;

pub fn player_box(name: &str) -> Box {
    let title = label("title", "title");
    let artist = label("artist", "artist");
    let artpic = img_box_simple("artpic", ARTPIC_BASE, ARTPIC_BASE, "images/lavender.jpg");

    let title_clone = title.clone();
    let artist_clone = artist.clone();
    let artpic_clone = artpic.clone();

    let r = spawn_listen(FOLLOW_MPRIS_METADATA);

    glib::MainContext::default().spawn_local(async move{
        while let Ok(metadata_dirty) = r.recv().await {
            let metadata: Vec<&str> = metadata_dirty.split("%sep%").collect();

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

            for c in artpic_clone.children() {
                artpic_clone.remove(&c);
            }

            if metadata[2] != "" {
                let img_path = metadata[2].replace("file://", "");
                
                let cmd = String::from(GET_IMG_GEOMETRY) + &img_path;
                let out = exec_once(&cmd);
                let img_geom: Vec<&str> = out.split(" ").collect();

                let img_w = match img_geom[0].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => ARTPIC_BASE as f64,
                };

                let img_h = match img_geom[1].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => ARTPIC_BASE as f64,
                };

                let wratio: f64 = img_w / img_h;
                let width: f64 = wratio * ARTPIC_BASE as f64;

                artpic_clone.add(&img_box_simple("artpic", width as i32, ARTPIC_BASE, &img_path));
                artpic_clone.show_all();
            } else {
                artpic_clone.add(&img_box_simple("artpic", ARTPIC_BASE, ARTPIC_BASE, "images/lavender.jpg"));
            }
        }
    });

    let controller = Box::new("controller", H)
        .spacing(8)
        .cerberus(

        get_button("prev_button", PREV),
        get_button("toggle_button", TOGGLE),
        get_button("next_button", NEXT)
    );

    Box::new(name, H)
        .spacing(8)
        .load(vec![

        artpic,
        Box::new("", V).valign(Start).cerberus(
            Box::new("", H).child(title),
            sep("sep"),
            Box::new("", H).child(artist),
        ),
        pad(),
        controller,
    ])
}

fn get_button(name: &str, action: MprisAction) -> Button {
    let button = Button::new(name);

    match action {
        PREV => button.set_label("≪"),
        TOGGLE => button.set_label("≠"),
        NEXT => button.set_label("≫"),
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
