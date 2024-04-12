use pastry::widgets::{
    custom_window::Window,
    custom_box::Box,
    custom_button::Button,
};

use pastry::factory::{
    shell::{
        poll_label,
        listen_label,
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
use gtk::Orientation::{
    Horizontal as H,
    Vertical as V,
};

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
    
    Box::new("bar_main", V).load(vec![
        Box::new("", V).child(
            btn1,
        ),
        pad(),
        Box::new("bar_hp", V).load(vec![
            label("", "HP"),
            bat_cap_label(""),
        ]),
        sep("bar_sep"),
        Box::new("bar_clock", V).load(vec![
            poll_label("", "date +\"%H\"", 30 as u64), 
            label("", "時"),
            poll_label("", "date +\"%M\"", 30 as u64),
            label("", "分"),
        ]),
    ])
}

fn ui_card() -> Box {
    Box::new("card_main", V)
    .load(vec![
        Box::new("card_title", V).child(
            poll_label("clock", "date +'%A %b %d, %4Y'", 60)
        ),
        Box::new("card_body", V).spacing(8).load(vec![
            user_info(),
            system_info(),
            web_bookmarks(),
        ]),
        Box::new("card_bottom", V).load(vec![
            mpris_box()
        ]),
    ])
}

fn user_info() -> Box {
    Box::new("user_info", V).cerberus(
        img_box_simple("", 180, 180, "images/lavender.jpg"),
        sep(""),
        label("user_name", &exec_once("echo $USER@$(cat /etc/hostname)"))
    )
}

fn system_info() -> Box {
    Box::new("system_info", V).cerberus(
        label("subtitle", "System Info"),
        sep(""),
        Box::new("", V).load(vec![
            Box::new("", H).cerberus(
                label("", "Volume"),
                pad(),
                Box::new("", H).cerberus(
                    volume_label(""),
                    sep(""),
                    label("", "%")
                )
            ),
            Box::new("", H).cerberus(
                label("", "Brightness"),
                pad(),
                Box::new("", H).cerberus(
                    brightness_label(""),
                    sep(""),
                    label("", "%")
                )
            ),
            Box::new("", H).cerberus(
                label("", "Network"),
                pad(),
                network_label("")
            ),
        ])
    )
}

fn web_bookmarks() -> Box {
    let bookmarks = vec![
        (Button::new("bm1"), label("l1", "albo"), "https://www.chukyo-u.ac.jp/student-staff/it/cubics/"),
        (Button::new("bm2"), label("l2", "manabo"), "https://manabo.cnc.chukyo-u.ac.jp/auth/shibboleth/"),
        (Button::new("bm3"), label("l3", "github"), "https://github.com/"),
        (Button::new("bm4"), label("l4", "youtube"), "https://www.youtube.com/"),
    ];
 
    for (btn, _, url) in &bookmarks {
        btn.set_label(url);
    
        btn.connect_clicked(|btn|{
            spawn_once(&("xdg-open ".to_string() + url));
            if let Some(win) = btn.toplevel() {
                win.hide();
            }
        });
    }

    Box::new("web_bookmarks", V).cerberus(
        label("subtitle", "Bookmarks"),
        sep(""),
        Box::new("", V)
            .halign(Center)
            .spacing(4)
            .load(
                bookmarks.into_iter()
                    .map(|(btn, l, _)| Box::new("", V)
                        .halign(Start)
                        .cerberus(
                            Box::new("", H).child(l),
                            sep(""),
                            btn
                        )
                    )
                    .collect()
            )
    )
}

fn mpris_box() -> Box {
    Box::new("mpris_box", V)
}
