#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;

mod config;


use std::fs::{self, File, DirBuilder, OpenOptions};
use std::path::{PathBuf, Path};
use std::io;

enum TargetType {
    Executable(ExecutableType),
    Directory,
    Archive,
    Unknown,
}

enum ExecutableType {
    Binary,
    ShellScript,
    AppImage,
    Other,
}

fn classify_target<A: AsRef<Path>>(path: A) -> Result<TargetType, io::Error> {
    use TargetType::*;
    use ExecutableType::*;
    if fs::metadata(&path)?.is_dir() {
        return Ok(Directory);
    }

    if let Some(ext) = path.as_ref().extension() {
        match &*ext.to_string_lossy() {
            "sh" => { return Ok(Executable(ShellScript)); },
            "appimage" => { return Ok(Executable(AppImage)); },
            _ => {},
        }
    }

    Ok(Unknown)
}