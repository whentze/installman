use std::path::{Path, PathBuf};
use std::env;

lazy_static! {
    pub static ref CONFIG_LOCATION : PathBuf = {
        let mut path = env::home_dir().unwrap();
        path.push(".config/installman/config.toml");
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
}

impl Config {
    pub fn new<A: AsRef<Path>>(location: A) -> Self {
        Config {
            apps_location : location.as_ref().to_path_buf()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut path = env::home_dir().unwrap();
        path.push("installman_apps");
        Config::new(path)
    }
}