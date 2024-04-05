mod imp;

use gtk::glib;
use gtk::gdk;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;

}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::builder()
            .property("application", app)
            .build()
    }

    pub fn new_widget(app: &gtk::Application,
        name: &str,
        x: i32, y: i32,
        w: i32, h: i32
    ) -> Self {

        let new_window: Window = glib::Object::builder()
            .property("type", gtk::WindowType::Popup)
            .build();

        new_window.set_application(Some(app));
        new_window.set_widget_name(name);
        new_window.set_decorated(true);

        new_window.move_(x, y);
        new_window.set_default_size(w, h);

        new_window.set_resizable(false);
        new_window.set_accept_focus(false);

        new_window.set_skip_pager_hint(true);
        new_window.set_skip_taskbar_hint(true);
        new_window.set_type_hint(gdk::WindowTypeHint::Dock);
        new_window.stick();
 
        new_window
    }
}
