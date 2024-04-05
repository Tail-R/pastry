use pastry::widgets::{
    custom_window::Window,
    custom_box::Box,
    custom_button::Button,
};

use pastry::factory::{
    shell::*,
    system::bat_label,
    misc::*,
};

use std::env;

use gtk::prelude::*;
use gtk::Orientation::*;

pub fn build(app: &gtk::Application) {
    let card = Window::new_widget(app, "card", 70, 20, 600, 400);
    let bar = Window::new_widget(app, "win", 0, 0, 50, 1200);
 
    /* Bar configuration */
    bar.add(&ui_bar(card.clone())); 
    bar.show_all();
    
    /* Card configuration */
    card.add(&ui_card());
}

fn ui_bar(card: Window) -> Box {
    let btn1 = Button::new("bar_btn1");
    btn1.set_label("★");
     
    btn1.connect_clicked(move |_| {
        if card.is_visible() {
            card.hide();
        } else {
            card.show_all();
        }
    });
    
    Box::new("bar_main_box", Vertical).add_children(vec![
        &Box::new("bar_buttons", Vertical).add_children(vec![
            &btn1,
        ]),
        &pad(),
        &Box::new("bar_bat", Vertical).add_children(vec![
            &gtk::Label::new(Some("HP")),
            &bat_label("")
        ]),
        &sep("bar_sep"),
        &Box::new("bar_date", Vertical).add_children(vec![
            &poll_label("", "date +\"%H\"", 30 as u64), 
            &gtk::Label::new(Some("時")),
            &poll_label("", "date +\"%M\"", 30 as u64),
            &gtk::Label::new(Some("分")),
        ]),
    ])
}

fn ui_card() -> Box {
    let home = env::var("HOME").unwrap();

    let user_name = gtk::Label::new(Some(&exec_once("echo $USER")));
    user_name.set_widget_name("user_name");

    let host_name = gtk::Label::new(
        Some(&("@".to_string() + &exec_once("cat /etc/hostname")))
    );
    host_name.set_widget_name("host_name");

    /* Card Body */
    let body = Box::new("card_body", Horizontal);
    body.set_vexpand(true);

    let body_start = Box::new("card_b_start", Vertical).add_children(vec![
        &img_box("", 180, 180, &(home + "/Pictures/satori.jpg")),
        &Box::new("", Vertical).add_children(vec![
            &user_name,
            &host_name,
        ]),
    ]);
    
    let body_center = Box::new("card_b_center", Vertical);
    body_center.set_hexpand(true);

    let body_end = Box::new("card_b_end", Vertical);

    body.add_children(vec![
        &body_start,
        &body_center,
        &body_end,
    ]);

    Box::new("card_main_box", Vertical).add_children(vec![
        &Box::new("card_title", Horizontal),
        &body,
        &Box::new("card_bottom", Horizontal),
    ])
}
