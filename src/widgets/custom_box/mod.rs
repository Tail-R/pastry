mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::Orientation::*;

glib::wrapper! {
    pub struct Box(ObjectSubclass<imp::Box>)
        @extends gtk::Widget, gtk::Container, gtk::Box;
}

impl Box {
    pub fn new(name: &str, orientation: gtk::Orientation) -> Self {
        if orientation ==  Horizontal {
            glib::Object::builder()
                .property("name", name)
                .property("orientation", Horizontal)
                .build()
        } else {
            glib::Object::builder()
                .property("name", name)
                .property("orientation", Vertical)
                .build()
        }
    }

    pub fn add_children<T: IsA<gtk::Widget>>(&self, children: Vec<&T>) -> Self {
        for c in children {
            self.add(c);
        }

        self.clone()
    }
}
