mod config;

use gtk::gdk;
use gtk::prelude::*;

const APP_ID: &str = "org.pastry";

fn load_css() {
    let provider = gtk::CssProvider::new();
    let style = include_bytes!("style.css");
    
    provider.load_from_data(style).expect("Failed to load css");
    
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Failed to init css provider"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

fn main() {
    let app = gtk::Application::new(Some(APP_ID), Default::default());

    app.connect_activate(|app| {
        config::build(app);
        load_css();
    });
    
    app.run();
}
