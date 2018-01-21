extern crate gtk;
extern crate installman;

use gtk::prelude::*;
use installman::lib::install_target;
use installman::config::DATA;
use installman::error::*;
use std::sync::mpsc::channel;
use std::path::PathBuf;
use std::sync::RwLock;

fn main() {
    installman::lib::init().unwrap();
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    //Init main_window
    let glade_src = include_str!("gui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let button_install: gtk::Button = builder.get_object("button_install").unwrap();
    let button_uninstall: gtk::Button = builder.get_object("button_uninstall").unwrap();
    let file_chooser: gtk::FileChooser = builder.get_object("file_chooser").unwrap();
    let list_store: gtk::ListStore = builder.get_object("list_store").unwrap();
    let tree_view: gtk::TreeView = builder.get_object("tree_view").unwrap();
    let label_file_chooser: gtk::Label = builder.get_object("label_file_chooser").unwrap();
    let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
    for element in DATA.read().unwrap().installed_apps.iter() {
        list_store.insert_with_values(Some(0), &[0, 1], &[&element.name, &"lol".to_string()]);
    }


    //Init AlreadyInstalledDialog
    let dialog: gtk::Dialog = builder.get_object("dialog_already_installed").unwrap();
    let radio_button_1: gtk::RadioButton = builder.get_object("dialog_already_installed_radiobutton_1").unwrap();
    let text_entry: gtk::Entry = builder.get_object("dialog_already_installed_textentry").unwrap();
    let button_cancel: gtk::Button = builder.get_object("dialog_already_installed_cancel").unwrap();
    let button_ok: gtk::Button = builder.get_object("dialog_already_installed_ok").unwrap();

    let (tx, rx) = channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    let (s_gui_msg, r_gui_msg) = channel();
    let s_gui_msg2 = s_gui_msg.clone();
    let s_gui_msg3 = s_gui_msg.clone();
    let s_gui_msg4 = s_gui_msg.clone();


    let dialog2 = dialog.clone();
    let dialog3 = dialog.clone();
    let dialog4 = dialog.clone();

    let radio_button_1_2 = radio_button_1.clone();
    let radio_button_1_3 = radio_button_1.clone();

    let text_entry2 = text_entry.clone();
    let text_entry3 = text_entry.clone();

    let file_chooser2 = file_chooser.clone();

    let list_store2 = list_store.clone();
    let list_store3 = list_store.clone();
    let list_store4 = list_store.clone();

    let label_file_chooser2 = label_file_chooser.clone();

    let tree_view2 = tree_view.clone();

    let mut overwrite_data = (String::new(), String::new());

    //Connect main_window functions
    button_uninstall.connect_clicked(move |_|{
        let tree_selection = tree_view.get_selection();
        let x = tree_selection.get_selected();
        match x{
            Some(_) =>{
                let x1 = x.clone();
                let y = x1.unwrap().1;
                let value = x.unwrap().0.get_value(&y, 0);
                installman::lib::uninstall_target(&value.get::<String>().unwrap()).unwrap();
                list_store3.remove(&y);
            },
            None => {}
        }


    });

    button_install.connect_clicked(move |_| {
        match file_chooser.get_filename()
            {
                Some(s) => install(s.clone(), installman::lib::get_app_name(s).unwrap(), s_gui_msg3.clone(),
                                   list_store.clone(), text_entry.clone(),
                                   radio_button_1.clone(), dialog.clone(), label_file_chooser.clone()),
                None => label_file_chooser.set_text("Please Select An App"),
            }
    });

    button_ok.connect_clicked(move |_| {
        dialog_ok(s_gui_msg.clone() ,radio_button_1_2.clone(), text_entry2.clone(), dialog2.clone(), tx2.clone());
    });

    //Connect AlreadyInstalledDialog functions
    button_cancel.connect_clicked(move |_| {
        tx3.send(AlreadyInstalledDecision::Cancel).unwrap();
        dialog3.hide();
    });

    window.show_all();
    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    while (gtk::main_iteration()) {
        use AlreadyInstalledDecision::*;
        use GUIMessage::*;
        match rx.try_recv() {
            Ok(Overwrite) => {
                installman::lib::uninstall_target(&overwrite_data.0.clone());
                let model = tree_view2.get_model();
                let mut i = model.clone().unwrap().get_iter_first();
                loop{
                    if model.clone().unwrap().get_value(&i.clone().unwrap(),0).get::<String>().unwrap() == overwrite_data.0{
                        list_store4.remove(&i.clone().unwrap());
                        break;
                    }
                    if !model.clone().unwrap().iter_next(&i.clone().unwrap()){
                        break;
                    }
                }
                install(std::path::PathBuf::from(&overwrite_data.1), overwrite_data.0.clone(), s_gui_msg2.clone(),
                        list_store2.clone(), text_entry3.clone(),
                        radio_button_1_3.clone(), dialog4.clone(), label_file_chooser2.clone());
            }
            Ok(NewName(name)) => {
                //redundant
                match file_chooser2.get_filename()
                {
                    Some(filename) => {
                        install(filename.clone(), name, s_gui_msg2.clone(),
                                list_store2.clone(), text_entry3.clone(),
                                radio_button_1_3.clone(), dialog4.clone(), label_file_chooser2.clone());
                    },
                    None => label_file_chooser2.set_text("Please Select An App"),
                }
            }
            Ok(Cancel) => dialog4.hide(),
            Err(e) => {}
        }
        match r_gui_msg.try_recv(){
            Ok(StoreOverwrite(n, p)) => {
                overwrite_data = (n, p);
            },
            Err(e) => {},
        }
    }
    return;
}

#[derive(Debug)]
enum AlreadyInstalledDecision {
    Overwrite,
    NewName(String),
    Cancel,
}

#[derive(Debug)]
enum GUIMessage {
    StoreOverwrite(String, String),
}


fn install(path: PathBuf, name: String, s_gui_msg: std::sync::mpsc::Sender< GUIMessage>, list_store: gtk::ListStore, text_entry: gtk::Entry, radio_button_1: gtk::RadioButton, dialog: gtk::Dialog, label_file_chooser: gtk::Label) -> () {
    match install_target(path.clone(), name.clone()) {
        Ok(y) => {
            list_store.insert_with_values(Some(0), &[0, 1], &[&y, &"01.01.2100".to_string()]);
        }
        Err(Error(AlreadyInstalledApp(_), _)) => {
            s_gui_msg.send(GUIMessage::StoreOverwrite( name,path.to_str().unwrap().to_string()));
            text_entry.set_text("");
            radio_button_1.set_active(true);
            dialog.show_all();
        }
        Err(Error(TargetTypeNotSupported, _)) => label_file_chooser.set_text("Target type is not supported!"),
        Err(e) => {
            label_file_chooser.set_text("Installation Failed!");
        }
    }
}

fn dialog_ok (s_gui_msg: std::sync::mpsc::Sender< GUIMessage>, radio_button_1: gtk::RadioButton, text_entry: gtk::Entry, dialog: gtk::Dialog, tx: std::sync::mpsc::Sender< AlreadyInstalledDecision > ) -> (){
    if radio_button_1.get_active() {
        // TODO: What to do if the new name is empty/also already exists
        tx.send(AlreadyInstalledDecision::NewName(text_entry.get_text().unwrap())).unwrap();
    } else {
        tx.send(AlreadyInstalledDecision::Overwrite).unwrap();
    }
    dialog.hide();
}