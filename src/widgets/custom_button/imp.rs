use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct Button;

#[glib::object_subclass]
impl ObjectSubclass for Button {
    const NAME: &'static str = "CustomButton";
    type Type = super::Button;
    type ParentType = gtk::Button;
}

impl ObjectImpl for Button {}
impl WidgetImpl for Button {}
impl ContainerImpl for Button {}
impl ButtonImpl for Button {}
impl BinImpl for Button {}
