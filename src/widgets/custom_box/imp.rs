use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct Box;

#[glib::object_subclass]
impl ObjectSubclass for Box {
    const NAME: &'static str = "CustomBox";
    type Type = super::Box;
    type ParentType = gtk::Box;
}

impl ObjectImpl for Box {}
impl WidgetImpl for Box {}
impl ContainerImpl for Box {}
impl BoxImpl for Box {}
