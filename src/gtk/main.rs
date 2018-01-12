extern crate gtk;
extern crate installman;
use gtk::prelude::*;
use installman::lib::{classify_target, install_target, TargetType};


fn main() {
    installman::lib::init();
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


    button_install.connect_clicked(move|_|{
        //match file_chooser.get_filename(){
            //debug code
            //Some(x) => match classify_target(x) {
            //    Ok(v) => label_file_chooser.set_text(&format!("File identified as: {}", v)),
            //    Err(_) =>
            //},
            //None => label_file_chooser.set_text("Please select a file!")
        //}
        match install_target(file_chooser.get_filename().unwrap()){
            Ok(x) => {
                list_store.insert_with_values(Some(0), &[0, 1], &[&x, &"01.01.2100".to_string()]);
            },
            Err(_) => label_file_chooser.set_text("Installation Failed!"),
        }
    });

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}
