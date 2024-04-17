mod imp;

use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Scroll(ObjectSubclass<imp::Scroll>)
        @extends gtk::Widget, gtk::Container, gtk::ScrolledWindow, gtk::Bin;
}

impl Scroll {
    pub fn new(name: &str) -> Self {
        let new_scroll: Scroll = glib::Object::new();

        new_scroll.set_widget_name(name);
        new_scroll.set_hexpand(true);
        new_scroll.set_vexpand(true);

        new_scroll
    }

    pub fn child<T: IsA<gtk::Widget>>(&self, child: T) -> Self {
        self.add(&child);

        self.clone()
    }
}
