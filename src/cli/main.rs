#[macro_use]
extern crate clap;
extern crate installman;

use clap::{Arg, SubCommand};
use installman::classify_target;

fn main() {
    let matches = app_from_crate!()
        .subcommand(SubCommand::with_name("install")
                        .about("install an app")
                        .arg(Arg::with_name("file")
                                 .required(true)
                                 .index(1)
                                 .help("file to install from")))
        .subcommand(SubCommand::with_name("list").about("list installed apps"))
        .subcommand(SubCommand::with_name("remove")
                        .about("uninstall an app")
                        .arg(Arg::with_name("app")
                                 .required(true)
                                 .index(1)
                                 .help("name of the app to remove")))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("install") {
        let target = matches.value_of("file").unwrap();
        match classify_target(target){
            Ok(x) => println!("File identified as: {}", x),
            Err(_) => println!("Target Classification Failed!"),
        }
    }
}
