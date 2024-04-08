mod config;

use gtk::gdk;
use gtk::prelude::*;

const APP_ID: &str = "org.pastry";

/*
---------- To-Do ----------
use DrawingArea instead of GtkImage in img_box()
*/

fn load_css() {
    let provider = gtk::CssProvider::new();

    if let Ok(css) = grass::from_path("style.scss", &grass::Options::default()) {
        let style: &[u8] = css.as_bytes();
        
        if let Ok(_) = provider.load_from_data(style) {
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::default().expect("Failed to init css provider"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
            );
        }
    }
}

fn main() {
    let app = gtk::Application::new(Some(APP_ID), Default::default());

    app.connect_activate(|app| {
        config::build(app);
        load_css();
    });
    
    app.run();
}
