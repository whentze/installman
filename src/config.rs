use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::ffi::OsString;
use std::io::Write;
use toml;
use error::*;

lazy_static! {
    static ref CONFIG_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/config.toml");
        path
    };
    static ref DATA_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/data.toml");
        path
    };
    static ref APPS_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("installman_apps");
        path
    };
    static ref DESKTOP_FILES_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".local/share/applications");
        path
    };
    static ref BIN_SYMLINK_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("bin");
        path
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct App{
    name: OsString,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Data {
    installed_apps: Vec<App>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    apps_location: PathBuf,
    data_location: PathBuf,
    desktop_files_location: PathBuf,
    bin_symlink_location: PathBuf,
}

impl Config {
    fn new<A: AsRef<Path>>(apps: A, data: A, desk: A, bins: A) -> Self {
        Config {
            apps_location: apps.as_ref().to_path_buf(),
            data_location: data.as_ref().to_path_buf(),
            desktop_files_location: desk.as_ref().to_path_buf(),
            bin_symlink_location: bins.as_ref().to_path_buf(),
        }
    }

    fn store(&self) -> Result<()> {
        let mut f = File::create(&*CONFIG_LOCATION)?;
        f.write(&*toml::to_vec(self)?)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(&*APPS_LOCATION,
                    &*DATA_LOCATION,
                    &*DESKTOP_FILES_LOCATION,
                    &*BIN_SYMLINK_LOCATION)
    }
}