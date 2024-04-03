use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct Window;

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "CustomWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for Window {}
impl WidgetImpl for Window {}
impl ContainerImpl for Window {}
impl BinImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
