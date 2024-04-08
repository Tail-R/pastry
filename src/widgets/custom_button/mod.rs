mod imp;

use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Button(ObjectSubclass<imp::Button>)
        @extends gtk::Widget, gtk::Container, gtk::Button, gtk::Bin;
}

impl Button {
    pub fn new(name: &str) -> Self {
        let new_button: Button = glib::Object::new();

        new_button.set_widget_name(name);

        new_button
    }

    pub fn add_child<T: IsA<gtk::Widget>>(&self, child: T) -> Self {
        self.add(&child);

        self.clone()
    }
}
