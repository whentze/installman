extern crate gtk;
extern crate installman;

use gtk::prelude::*;
use installman::lib::install_target;

fn main() {
    installman::lib::init().unwrap();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("gui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let button_install: gtk::Button = builder.get_object("button_install").unwrap();
    let file_chooser: gtk::FileChooser = builder.get_object("file_chooser").unwrap();
    let list_store: gtk::ListStore = builder.get_object("list_store").unwrap();
    let label_file_chooser: gtk::Label = builder.get_object("label_file_chooser").unwrap();
    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    window.show_all();


    button_install.connect_clicked(move |_| {
        match file_chooser.get_filename() {
            Some(x) => match install_target(x) {
                Ok(y) => {
                    list_store.insert_with_values(Some(0), &[0, 1], &[&y, &"01.01.2100".to_string()]);
                }
                Err(e) => {label_file_chooser.set_text("Installation Failed!");
                            eprintln!("{:?}", e);
                },
            }
            None => (),
        };
    });

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
    return;
}
