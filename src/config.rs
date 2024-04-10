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
        img_box_simple,
        pad,
        sep,
    },
};

use gtk::prelude::*;
use gtk::Orientation::*;
use gtk::Align::*;

pub fn build(app: &gtk::Application) {
    let bar = Window::new_widget(app, "", 0, 0, 50, 1200);
    let card = Window::new_widget(app, "", 70, 20, 0, 0);
 
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
        Box::new("bar_clock", Vertical).add_children(vec![
            poll_label("", "date +\"%H\"", 30 as u64), 
            label("", "時"),
            poll_label("", "date +\"%M\"", 30 as u64),
            label("", "分"),
        ]),
    ])
}

fn ui_card() -> Box { 
    Box::new("card_main", Vertical).add_children(vec![
        Box::new("card_title", Horizontal).homogeneous(true).add_children(vec![
            poll_label("date", "date +'%A %b %d, %4Y'", 60)
        ]),
        Box::new("card_header", Horizontal).homogeneous(true).add_child(
            pad()
        ),
        Box::new("card_body", Horizontal).homogeneous(true).add_children(vec![
            img_box_simple("pfp_box", 180, 180, &"images/lavender.jpg"),
            user_info(),
            sys_info(),
        ]),
        Box::new("card_bottom", Vertical).add_children(vec![
            web_bookmarks(),
        ]),
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

    Box::new("user_info", Vertical)
        .homogeneous(true)
        .add_children(vec![

        Box::new("subtitle", Horizontal).halign(Center).add_child(
            label("", "Profile")
        ),

        Box::new("user_name", Horizontal).cerberus(
            label("", "User"),
            pad(),
            label("", "Tail-R"), // Hard coding is my beloved (u_u*)
        ),

        Box::new("host_name", Horizontal).cerberus(
            label("", "Host"),
            pad(),
            label("", &exec_once("cat /etc/hostname")),
        ),

        Box::new("term_cols", Horizontal)
            .homogeneous(false)
            .halign(Center)
            .add_children(

            base_8,
        ),
        Box::new("uptime", Vertical)
            .homogeneous(true)
            .add_child(

            poll_label("", "uptime -p | cut -d ',' -f1", 60)
        ),
    ])
}

fn sys_info() -> Box {
    Box::new("sys_info", Vertical)
        .homogeneous(true)
        .add_children(vec![

        Box::new("subtitle", Horizontal).halign(Center).add_child(
            label("", "System")
        ),

        Box::new("brightness", Horizontal).cerberus(
            label("", "Brightness"),
            pad(),
            Box::new("", Horizontal).add_children(vec![
                brightness_label("value"),
                label("icon", "%"),
            ]),
        ),
        
        Box::new("volume", Horizontal).cerberus(
            label("", "Volume"),
            pad(),
            Box::new("", Horizontal).add_children(vec![
                volume_label("value"),
                label("icon", "%"),
            ]),
        ),

        Box::new("network", Vertical)
            .valign(Center)
            .cerberus(

            Box::new("", Horizontal).add_child(
                label("", "Network")
            ),
            pad(),
            Box::new("", Horizontal).homogeneous(true).add_child(
                network_label("ap_name")
            ),
        )
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
    
        btn.connect_clicked(|btn|{
            spawn_once(&("xdg-open ".to_string() + url));
            if let Some(win) = btn.toplevel() {
                win.hide();
            }
        });
    }

    Box::new("web_bookmarks", Vertical)
        .homogeneous(true)
        .spacing(4)
        .add_children(
            bookmarks.into_iter()
                .map(|(btn, _)| Box::new("", Horizontal).add_child(btn))
                .collect()
        )
}
