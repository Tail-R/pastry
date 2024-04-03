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

use gtk::prelude::*;
use gtk::Orientation::*;

pub fn build(app: &gtk::Application) {
    let bar = Window::new(app, "win", 0, 0, 40, 1200);
 
    let b1 = Button::new("bt_1"); 
    let b2 = Button::new("bt_2");
    let b3 = Button::new("bt_3");
    let b4 = Button::new("bt_4");
    
    b1.connect_clicked(|_| {
        spawn_once("xdg-open https://www.chukyo-u.ac.jp/student-staff/it/cubics/");
    });
    
    b2.connect_clicked(|_| {
        spawn_once("xdg-open https://manabo.cnc.chukyo-u.ac.jp/login/shibboleth/");
    });

    b3.connect_clicked(|_| {
        spawn_once("xdg-open https://www.youtube.com/");
    });

    b4.connect_clicked(|_| {
        spawn_once("xdg-open https://github.com/");
    });
 
    bar.add(
        &Box::new("v_box", Vertical).add_children(vec![
            &Box::new("buttons", Vertical).add_children(vec![
                &b1,
                &b2,
                &b3,
                &b4,
            ]),
            &pad(),
            &Box::new("bat", Vertical).add_children(vec![
                &gtk::Label::new(Some("HP")),
                &bat_label("cap")
            ]),
            &sep("sep"),
            &Box::new("date", Vertical).add_children(vec![
                &poll_label("hour", "date +\"%H\"", 30 as u64), 
                &gtk::Label::new(Some("時")),
                &poll_label("min", "date +\"%M\"", 30 as u64),
                &gtk::Label::new(Some("分")),
            ]),
        ]),
    );
    
    bar.show_all();
}
