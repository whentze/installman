extern crate gtk;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("../gtk/main.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window:  gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    let scale:   gtk::Scale             = builder.get_object("goennungs_scale").unwrap();
    let button:  gtk::CheckButton       = builder.get_object("extra_goennung_button").unwrap();
    let warning: gtk::Label             = builder.get_object("warnungs_label").unwrap();

    window.show_all();

    let scale_clone = scale.clone();
    let orig_fill_level = scale.get_fill_level();
    button.connect_toggled(move |but| {
        if but.get_active() {
            scale_clone.set_fill_level(std::f64::MAX);
        } else {
            scale_clone.set_fill_level(orig_fill_level);
        }
    });

    scale.connect_value_changed(move |sca| {
        warning.set_visible(sca.get_value() > 90.0);
    });

    window.connect_delete_event(move |w, _| {
            gtk::main_quit();
            Inhibit(false)
    });

    gtk::main();
}
