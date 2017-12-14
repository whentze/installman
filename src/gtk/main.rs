extern crate gtk;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("gui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.show_all();

    let list_store: gtk::ListStore = builder.get_object("list_store").unwrap();

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
