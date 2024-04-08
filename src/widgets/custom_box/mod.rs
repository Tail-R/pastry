mod imp;

use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Box(ObjectSubclass<imp::Box>)
        @extends gtk::Widget, gtk::Container, gtk::Box;
}

impl Box {
    pub fn new(name: &str, orientation: gtk::Orientation) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("orientation", orientation)
            .build()
    }

    pub fn add_child<T: IsA<gtk::Widget>>(&self, child: T) -> Self {
        self.add(&child);

        self.clone()
    }

    pub fn add_children<T: IsA<gtk::Widget>>(&self, children: Vec<T>) -> Self {
        for c in children {
            self.add(&c);
        }

        self.clone()
    }

    pub fn cerberus<T1: IsA<gtk::Widget>, T2: IsA<gtk::Widget>, T3: IsA<gtk::Widget>>(
        &self,
        c1: T1,
        c2: T2,
        c3: T3,
    ) -> Self {

        self.pack_start(&c1, false, false, 0);
        self.add(&c2);
        self.pack_end(&c3, false, false, 0);

        self.clone()
    }

    pub fn name(&self, name: &str) -> Self {
        self.set_widget_name(name);

        self.clone()
    }

    pub fn homogeneous(&self, b: bool) -> Self {
        self.set_homogeneous(b);

        self.clone()
    }

    pub fn spacing(&self, n: i32) -> Self {
        self.set_spacing(n);

        self.clone()
    }

    pub fn halign(&self, align: gtk::Align) -> Self {
        self.set_halign(align);

        self.clone()
    }

    pub fn valign(&self, align: gtk::Align) -> Self {
        self.set_valign(align);

        self.clone()
    }

    pub fn hexpand(&self, b: bool) -> Self {
        self.set_hexpand(b);

        self.clone()
    }

    pub fn vexpand(&self, b: bool) -> Self {
        self.set_vexpand(b);

        self.clone()
    }
}
