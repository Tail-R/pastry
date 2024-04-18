mod config;

use gtk::gdk;
use gtk::gio;
use gtk::prelude::*;

const APP_ID: &str = "com.github.gtk-rs.pastry";
const STYLE_NAME: &str = "style.scss";

/*
---------- To-Do ----------
Think of ways to make the configuration more shorter ToT
I think it's better to load lua or static ML
and parse it into Rust structures.

Add wayland support. it's easy \(u_u*\)

Accept custom options from the command line
*/

fn load_css(path: Option<String>) {
    let style_target = match path {
        Some(cfg_dir) => format!("{}/{}", cfg_dir, STYLE_NAME),
        None => STYLE_NAME.to_string()
    };

    dbg!(&style_target);

    let provider = gtk::CssProvider::new();

    if let Ok(css) = grass::from_path(style_target, &grass::Options::default()) {
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
    let app = gtk::Application::new(
        Some(APP_ID),
        gio::ApplicationFlags::HANDLES_OPEN
    );

    // Activation that without path arguments
    // It doesn't load style seat
    app.connect_activate(|app| {
        config::build(app);
    });
 
    // If a configuration directory is given, open signal will be emitted
    app.connect_open(|app, files, _| {
        let argv = files.to_vec();

        let path_buf = match argv.len() {
            1 => argv[0].path().expect("No such path exists"),
            _ => panic!("Invalid number of parameters")
        };

        if let Some(path) = path_buf.to_str() {
            load_css(Some(path.to_string()));
        };

        config::build(app);
    });

    app.run();
}
