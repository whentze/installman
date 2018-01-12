#![feature(use_nested_groups)]
#[macro_use]
extern crate clap;
extern crate installman;

use clap::{Arg, ArgMatches, SubCommand, AppSettings};
use installman::{lib::classify_target, error::{err_msg, Result}};

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("install")
                        .about("install an app")
                        .arg(Arg::with_name("file")
                                 .required(true)
                                 .index(1)
                                 .help("file to install from")))
        .subcommand(SubCommand::with_name("list").about("list installed apps"))
        .subcommand(SubCommand::with_name("init").about("initialize files"))
        .subcommand(SubCommand::with_name("remove")
                        .about("uninstall an app")
                        .arg(Arg::with_name("app")
                                 .required(true)
                                 .index(1)
                                 .help("name of the app to remove")))
        .get_matches();

    if let (subcommand, Some(matches)) = matches.subcommand() {
        run_subcommand(subcommand, matches).unwrap_or_else(|e| {
            eprintln!("Unhandled error: {}", e);
            std::process::exit(1);
        });
    }
}

fn run_subcommand(subcommand: &str, matches: &ArgMatches) -> Result<()> {
    match subcommand {
        "install" => {
            let target = matches.value_of("file").unwrap();
            println!("File identified as: {}", classify_target(target)?);
            installman::lib::install_target(target)?;
        },
        "init" => {
            installman::lib::init()?
        },
        _ => return Err(err_msg("Unknown subcommand")),
    };
    Ok(())
}