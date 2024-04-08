use pastry::widgets::{
    custom_window::Window,
    custom_box::Box,
    custom_button::Button,
};

use pastry::factory::{
    shell::{
        poll_label,
        exec_once,
        spawn_once,
    },
    system::{
        volume_label,
        bat_cap_label,
        brightness_label,
        network_label,
    },
    misc::{
        label,
        img_box,
        pad,
        sep,
    },
};

use std::env;

use gtk::prelude::*;
use gtk::Orientation::*;
use gtk::Align::*;

pub fn build(app: &gtk::Application) {
    let card = Window::new_widget(app, "card", 70, 20, 0, 0);
    let bar = Window::new_widget(app, "bar", 0, 0, 50, 1200);
 
    bar.add(&ui_bar(card.clone())); 
    bar.show_all();
    card.add(&ui_card());
}

fn ui_bar(card: Window) -> Box {
    let btn1 = Button::new("draw_card");
    btn1.set_label("★");
     
    btn1.connect_clicked(move |_| {
        if card.is_visible() {
            card.hide();
        } else {
            card.show_all();
        }
    });
    
    Box::new("bar_main", Vertical).add_children(vec![
        Box::new("", Vertical).add_child(
            btn1,
        ),
        pad(),
        Box::new("bar_hp", Vertical).add_children(vec![
            label("", "HP"),
            bat_cap_label(""),
        ]),
        sep("bar_sep"),
        Box::new("bar_date", Vertical).add_children(vec![
            poll_label("", "date +\"%H\"", 30 as u64), 
            label("", "時"),
            poll_label("", "date +\"%M\"", 30 as u64),
            label("", "分"),
        ]),
    ])
}

fn ui_card() -> Box { 
    let home = env::var("HOME").unwrap();

    Box::new("card_main", Vertical).add_children(vec![
        Box::new("", Horizontal).homogeneous(true).add_children(vec![
            img_box("", 160, 160, &(home + "/Pictures/lavender.jpg")),
            user_info(),
            sys_info(),
        ]),
        web_bookmarks(),
    ])
}

fn user_info() -> Box {
    let base_8 = vec![
        label("c1", "■"),
        label("c2", "■"),
        label("c3", "■"),
        label("c4", "■"),
        label("c5", "■"),
        label("c6", "■"),
        label("c7", "■"),
        label("c8", "■"),
    ];

    Box::new("card_user_info", Vertical)
        .valign(Center)
        .spacing(16)
        .add_children(vec![

        Box::new("", Vertical).add_children(vec![
            label("user_name", "Tail-R"), // Hard coding is my beloved (u_u*)
            label("host_name", &("@".to_string() + &exec_once("cat /etc/hostname"))),
        ]),
        Box::new("term_cols", Horizontal)
            .homogeneous(true)
            .halign(Center)
            .spacing(0)
            .add_children(

            base_8,
        ),
        Box::new("uptime", Vertical)
            .homogeneous(true)
            .halign(Center)
            .add_child(

            poll_label("", "uptime -p | cut -d ',' -f1", 3600)
        ),
    ])
}

fn sys_info() -> Box {
    Box::new("card_sys_info", Vertical)
        .valign(Center)
        .spacing(8)
        .add_children(vec![

        Box::new("brightness", Horizontal)
            .halign(Center)
            .cerberus(

            label("", "・Brightness"),
            label("", ""),
            Box::new("", Horizontal).add_children(vec![
                brightness_label(""),
                label("", "%"),
            ]),
        ),
        
        Box::new("volume", Horizontal)
            .halign(Center)
            .cerberus(

            label("", "・Volume"),
            label("", ""),
            Box::new("", Horizontal).add_children(vec![
                volume_label(""),
                label("", "%"),
            ]),
        ),

        Box::new("network", Vertical)
            .halign(Center)
            .valign(Center)
            .add_children(vec![
                Box::new("", Horizontal).add_child(
                    label("", "・Network"),
                ),
                Box::new("", Horizontal).homogeneous(true).add_child(
                    network_label("ap_name"),
                ),
            ]),
    ])
}

fn web_bookmarks() -> Box {
    let bookmarks = vec![
        (Button::new("bm1"), "https://www.chukyo-u.ac.jp/student-staff/it/cubics/"),
        (Button::new("bm2"), "https://manabo.cnc.chukyo-u.ac.jp/auth/shibboleth/"),
        (Button::new("bm3"), "https://github.com/"),
        (Button::new("bm4"), "https://www.youtube.com/"),
    ];
 
    for (btn, url) in &bookmarks {
        btn.set_label(url);
    
        btn.connect_clicked(|_|{
            spawn_once(&("xdg-open ".to_string() + url));
        });
    }

    Box::new("web_bookmarks", Vertical)
        .homogeneous(true)
        .add_children(
            bookmarks.into_iter()
                .map(|(btn, _)| Box::new("", Horizontal).add_child(btn))
                .collect()
        )
}
