extern crate gtk;

use gtk::prelude::*;

use gtk::{Label, Window, WindowType, Inhibit};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Twitter");
    window.set_default_size(350, 70);
    let label = Label::new("Gnome Twitter");
    window.add(&label);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
