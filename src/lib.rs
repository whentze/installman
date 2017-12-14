#![feature(slice_patterns, advanced_slice_patterns)]

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
    let mut magic_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    let extension = path.extension().map(|e| e.to_string_lossy().into_owned());
    file.read_exact(&mut magic_bytes)?;

    Ok(match magic_bytes {
        [0x7F, b'E', b'L', b'F', ..]
            => Executable(match extension {
                Some(ref s) if s.to_lowercase() == "appimage" => AppImage,
                _ => Binary,
                }
            ),

        [b'#', b'!', ..]
            => Executable(Script),

        [0x1F, 0x8B, ..] |                    // .gz
        [0x1F, 0x9D, ..] | [0x1F, 0xA0, ..] | // .z
        [0x42, 0x5A, 0x68, ..]                // .bz2
            => Archive,

        [.., 0x00,   _,    _ ] |
        [.., b' ', b' ', 0x00] if &magic_bytes[..5] == b"ustar"
            => Archive,

        _ => Unknown,
    })
}
