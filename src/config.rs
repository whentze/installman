use std::path::{Path, PathBuf};
use std::env;

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