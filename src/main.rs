extern crate gtk;

use gtk::prelude::*;

use gtk::{Window, Inhibit, Builder};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("res/gtk/main.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: Window = builder.get_object("window").unwrap();
    window.show_all();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
