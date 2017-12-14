use std::path::{Path, PathBuf};
use std::env;

lazy_static! {
    pub static ref CONFIG_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/config.toml");
        path
    };
    pub static ref APPS_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("installman_apps");
        path
    };
    pub static ref DESKTOP_FILES_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".local/share/applications");
        path
    };
    pub static ref BIN_SYMLINK_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push("bin");
        path
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct App(PathBuf);

#[derive(Serialize, Deserialize, Debug)]
struct AppData {
    installed_apps: Vec<App>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    apps_location: PathBuf,
    desktop_files_location: PathBuf,
    bin_symlink_location: PathBuf,
}

impl Config {
    pub fn new<A:AsRef<Path>>(apps: A, desk: A, bins: A) -> Self {
        Config {
            apps_location          : apps.as_ref().to_path_buf(),
            desktop_files_location : desk.as_ref().to_path_buf(),
            bin_symlink_location   : bins.as_ref().to_path_buf(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(&*APPS_LOCATION, &*DESKTOP_FILES_LOCATION, &*BIN_SYMLINK_LOCATION)
    }
}