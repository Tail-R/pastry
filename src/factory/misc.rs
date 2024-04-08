use crate::widgets::{
    custom_box::Box,
};

use gtk::prelude::*;
use gtk::Orientation::*;
use gtk::gdk_pixbuf;

pub fn pad() -> Box {
    let pad = Box::new("", Horizontal);
    
    pad.set_hexpand(true);
    pad.set_vexpand(true);

    pad
}

pub fn sep(name: &str) -> Box {
    let sep = Box::new(name, Horizontal);
    
    sep.set_hexpand(false);
    sep.set_vexpand(false);

    sep
}

pub fn label(name: &str, label: &str) -> gtk::Label {
    gtk::Label::builder()
        .name(name)
        .label(label)
        .build()
}

pub fn img_box(
    name: &str,
    w: i32,
    h: i32,
    img_path: &str
    ) -> Box {

    let img_box = Box::new(name, Horizontal);
    img_box.set_halign(gtk::Align::Center);
    img_box.set_valign(gtk::Align::Center);

    let img_pb = gdk_pixbuf::Pixbuf::from_file(
        img_path
    ).expect("Image file does not found ToT");

    if let Some(pb) = img_pb.scale_simple(
        w, h, gdk_pixbuf::InterpType::Bilinear
    ) {
        img_box.add(&gtk::Image::builder()
            .pixbuf(&pb)
            .build());
    };

    img_box
}
