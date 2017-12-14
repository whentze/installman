#![feature(slice_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;

mod config;


use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Read};

#[derive(Debug)]
pub enum TargetType {
    Executable(ExecutableType),
    Directory,
    Archive,
    Unknown,
}

#[derive(Debug)]
pub enum ExecutableType {
    Binary,
    Script,
    AppImage,
    Other,
}

pub fn classify_target<A: AsRef<Path>>(path: A) -> Result<TargetType, io::Error> {
    use TargetType::*;
    use ExecutableType::*;
    let path = path.as_ref();
    if fs::metadata(path)?.is_dir() {
        return Ok(Directory);
    }

    let mut file = File::open(path)?;
    let mut magic_bytes : [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let extension = path.extension().map(|e| e.to_string_lossy().into_owned());
    file.read_exact(&mut magic_bytes)?;

    match magic_bytes {
        [0x7F, 0x45, 0x4C, 0x46, ..] => {
            match extension {
                Some(ref s) if s == "appimage" || s == "AppImage" => { return Ok(Executable(AppImage)); },
                _ => { return Ok(Executable(Binary)); },
            }
        }
        [0x1F, 0x8B, ..] | [0x1F, 0x9D, ..] | [0x1F, 0xA0, ..] |
        [0x42, 0x5A, 0x68, ..] |
        [0x75, 0x73, 0x74, 0x61, 0x72, 0x00, 0x30, 0x30] |
        [0x75, 0x73, 0x74, 0x61, 0x72, 0x20, 0x20, 0x00] => { return Ok(Archive); },
        [b'#', b'!', ..] => { return Ok(Executable(Script)); },
        _ => ()
    }
    Ok(Unknown)
}