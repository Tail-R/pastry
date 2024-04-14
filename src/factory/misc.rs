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
    x: i32,
    y: i32,
    w: f64,
    h: f64,
    img_path: &str
    ) -> Box {

    let drawing_area = gtk::DrawingArea::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    let img_pb = gdk_pixbuf::Pixbuf::from_file(
        img_path
    ).expect("Image file does not found ToT");

    drawing_area.connect_draw(move |_, cr| {
        cr.scale(w, h);
        cr.set_source_pixbuf(&img_pb, x.into(), y.into());
        let _ = cr.paint();

        true.into()
    });

    Box::new(name, Horizontal).child(drawing_area)
}

// pub fn img_box_pattern(
//     name: &str,
//     w: f64,
//     h: f64,
//     img_path: &str
//     ) -> Box {
// 
//     let x = 0;
//     let y = 0;
// 
//     let drawing_area = gtk::DrawingArea::builder()
//         .hexpand(true)
//         .vexpand(true)
//         .build();
// 
//     let img_pb = gdk_pixbuf::Pixbuf::from_file(
//         img_path
//     ).expect("Image file does not found ToT");
// 
//     drawing_area.connect_draw(move |_, cr| {
//         // cr.scale(w, h);
//         // cr.set_source_pixbuf(&img_pb, x.into(), y.into());
//         // let _ = cr.paint();
// 
//         // let pattern_surface = gtk::cairo::Surface();
// 
//         true.into()
//     });
// 
//     Box::new(name, Horizontal).child(drawing_area)
// }


// Easy to use, And works like a zombie OwO
pub fn img_box_simple(
    name: &str,
    w: i32,
    h: i32,
    img_path: &str
    ) -> Box {

    let img_box = Box::new(name, Horizontal);
    img_box.set_halign(gtk::Align::Center);
    img_box.set_valign(gtk::Align::Center);

    if let Ok(img_pb) = gdk_pixbuf::Pixbuf::from_file(img_path) {
        if let Some(pb) = img_pb.scale_simple(
            w, h, gdk_pixbuf::InterpType::Bilinear
        ) {
            img_box.add(&gtk::Image::builder()
                .pixbuf(&pb)
                .build());
        }
    }

    img_box
}
