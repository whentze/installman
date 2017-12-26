extern crate gtk;
extern crate installman;
use gtk::prelude::*;
use installman::{classify_target, TargetType};


fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("gui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let button_install: gtk::Button = builder.get_object("button_install").unwrap();
    let file_chooser: gtk::FileChooser = builder.get_object("file_chooser").unwrap();
    let label_file_chooser: gtk::Label = builder.get_object("label_file_chooser").unwrap();
    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.show_all();

    //let list_store: gtk::ListStore = builder.get_object("list_store").unwrap();

    button_install.connect_clicked(move|_|{
        match file_chooser.get_filename(){
            Some(x) => label_file_chooser.set_text(&format!("{:?}", classify_target(x))),
            None => label_file_chooser.set_text("Please select a file!")
        }

    });

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
