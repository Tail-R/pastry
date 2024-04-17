use pastry::widgets::{
    custom_window::Window,
    custom_box::Box,
    custom_button::Button,
    custom_scroll::Scroll,
};

use pastry::helper::get_substring;

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
    mpris::player_box,
    scripts::GET_IMG_GEOMETRY,
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
 
    bar.child(ui_bar(card.clone()));
    bar.show_all();


    card.child(ui_card());
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
        Box::new("bar_pad", V)
            .hexpand(true)
            .vexpand(true),
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
    Box::new("card_main", V).load(vec![
        Box::new("card_title", H)
            .spacing(16)
            .cerberus(

            img_box_simple("", 22, 22, "images/gtk.svg"),
            label("title", "まいぱねる"),
            sep("")
        ),
        Box::new("card_body", H)
            .homogeneous(true)
            .load(vec![

            Box::new("", V).load(vec![
                date(),
                user_info(),
                player_info(),
                system_info(),
            ]),
            Box::new("", V).load(vec![
                web_bookmarks(),
            ]),
        ]),
        Box::new("card_bottom", V),
    ])
}

fn date() -> Box {
    Box::new("date", V).child(
        poll_label("clock", "date +'%A %b %d, %4Y'", 60)
    )
}

fn user_info() -> Box {
    let colors = vec![
        label("c1", "◆"),
        label("c2", "◆"),
        label("c3", "◆"),
        label("c4", "◆"),
        label("c5", "◆"),
        label("c6", "◆"),
        label("c7", "◆"),
        label("c8", "◆"),
    ];

    let pfp_path = "images/scarlet.jpg";
    let pfp_h = 180.0;

    let cmd = String::from(GET_IMG_GEOMETRY) + pfp_path;
    let out = exec_once(&cmd);
    let geom: Vec<&str> = out.split(" ").collect();

    let (w, h) = match geom.len() {
        2 => {
            (
                geom[0].parse::<f64>().unwrap_or(1.0),
                geom[1].parse::<f64>().unwrap_or(1.0)
            )
        }
        _ => (1.0, 1.0),
    };

    let ratio: f64 = w / h;
    let pfp_w = ratio * pfp_h;

    Box::new("user_info", V)
        .spacing(8)
        .cerberus(

        img_box_simple("", pfp_w as i32, pfp_h as i32, pfp_path),
        label("user_name", &exec_once("echo $USER@$(cat /etc/hostname)")),
        Box::new("term_colors", H)
            .halign(Center)
            .spacing(8).load(

            colors
        )
    )
}

fn player_info() -> Box {
    Box::new("player_info", V).child(
        Scroll::new("").child(
            player_box("player_box")
        )
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
    let bookmark_max_len = 32;

    let bookmarks = vec![
        (Button::new("bm"), label("l", "university"), "https://www.chukyo-u.ac.jp/student-staff/it/cubics"),
        (Button::new("bm"), label("l", "class"), "https://manabo.cnc.chukyo-u.ac.jp/auth/shibboleth"),
        (Button::new("bm"), label("l", "github"), "https://github.com"),
        (Button::new("bm"), label("l", "youtube"), "https://www.youtube.com"),
        (Button::new("bm"), label("l", "gdrive"), "https://drive.google.com"),
        (Button::new("bm"), label("l", "keep"), "https://keep.google.com"),
        (Button::new("bm"), label("l", "reddit"), "https://www.reddit.com"),
        (Button::new("bm"), label("l", "discord"), "https://discord.com"),
        (Button::new("bm"), label("l", "spotify"), "https://open.spotify.com"),
        (Button::new("bm"), label("l", "pinterest"), "https://www.pinterest.jp"),
    ];
 
    for (btn, _, url) in &bookmarks {
        btn.set_label(&get_substring(url, bookmark_max_len));
    
        btn.connect_clicked(|btn|{
            spawn_once(&("xdg-open ".to_string() + url));
            if let Some(win) = btn.toplevel() {
                win.hide();
            }
        });
    }

    Box::new("web_bookmarks", V).cerberus(
        label("subtitle", "Bookmarks"),
        Scroll::new("scroll").child(
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
                        ).collect()
                )
        ),
        sep("")
    )
}
