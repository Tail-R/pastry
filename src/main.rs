mod config;

use gtk::gdk;
use gtk::prelude::*;

const APP_ID: &str = "org.pastry";

/*
---------- To-Do ----------
Think of ways to make the configuration more shorter ToT
I think it's better to load lua or static ML
and parse it into Rust structures.

Add wayland support. it's easy \(u_u*\)

Refactor ugly implementation of the MPRIS art box ~o~
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
