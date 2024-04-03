mod imp;

use gtk::glib;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;

}

impl Window {
    pub fn new(app: &gtk::Application,
        name: &str,
        x: i32, y: i32,
        w: i32, h: i32
    ) -> Self {

        let new_window: Window = glib::Object::new();

        new_window.set_application(Some(app));
        new_window.set_widget_name(name);
        new_window.set_decorated(false);

        new_window.move_(x, y);
        new_window.set_default_size(w, h);
        new_window.set_resizable(false);
        
        new_window.set_accept_focus(false);
        new_window.set_skip_pager_hint(true);
        new_window.set_skip_taskbar_hint(true);
        new_window.stick(); // send to all desktops

        new_window
    }
}
