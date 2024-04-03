use crate::widgets::{
    custom_box::Box,
};

use gtk::prelude::*;
use gtk::Orientation::*;

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
