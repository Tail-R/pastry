use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct Scroll;

#[glib::object_subclass]
impl ObjectSubclass for Scroll {
    const NAME: &'static str = "CustomScrolledWindow";
    type Type = super::Scroll;
    type ParentType = gtk::ScrolledWindow;
}

impl ObjectImpl for Scroll {}
impl WidgetImpl for Scroll {}
impl ContainerImpl for Scroll {}
impl ScrolledWindowImpl for Scroll {}
impl BinImpl for Scroll {}
