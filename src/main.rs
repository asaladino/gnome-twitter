extern crate gio;
extern crate gtk;

mod consts;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Builder, ApplicationWindow};
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../res/gtk/main.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window.");
    window.set_wmclass(consts::APP_NAME, consts::APP_NAME);
    window.set_application(application);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(consts::APP_ID, Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
