#![feature(slice_patterns, advanced_slice_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;
extern crate tar;

mod config;


use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Read};
use std::fmt;

pub enum TargetType {
    Executable(ExecutableType),
    Directory,
    Archive,
    Compressed(CompressionType),
    Unknown,
}

pub enum ExecutableType {
    Binary,
    Script,
    AppImage,
    Other,
}

pub enum CompressionType {
    Gzip,
    Bzip2,
    Lzw,
    Lzma,
    Unsupported,
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            &TargetType::Executable(ref Binary) => write!(f, "Executable Binary"),
            &TargetType::Executable(ref Script) => write!(f, "Executable Binary"),
            &TargetType::Executable(ref AppImage) => write!(f, "Executable Binary"),
            &TargetType::Executable(ref Other) => write!(f, "Executable Binary"),
            &TargetType::Directory => write!(f, "Directory"),
            &TargetType::Archive => write!(f, "Archive"),
            &TargetType::Compressed(ref Gzip) => write!(f, "Compressed Gzip"),
            &TargetType::Compressed(ref Lzw) => write!(f, "Compressed Lzw"),
            &TargetType::Compressed(ref Lzma) => write!(f, "Compressed Lzma"),
            &TargetType::Compressed(ref Unsupported) => write!(f, "Compressed Unsupported"),
            &TargetType::Unknown => write!(f, "Unknown Target Type"),
        }

    }
}

pub fn classify_target<A: AsRef<Path>>(path: A) -> Result<TargetType, io::Error> {
    use TargetType::*;
    use ExecutableType::*;
    use CompressionType::*;

    let path = path.as_ref();
    if fs::metadata(path)?.is_dir() {
        return Ok(Directory);
    }

    let mut file = File::open(path)?;
    let extension = path.extension().map(|e| e.to_string_lossy().into_owned());
    let mut magic_bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
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

        [0x1F, 0x8B, ..]
            => Compressed(Gzip),
        [0x1F, 0x9D, ..]
            => Compressed(Lzw),
        [0x42, 0x5A, 0x68, ..]
            => Compressed(Bzip2),
        [0xFD, b'7', b'z', b'X', b'Z', ..]
            => Compressed(Lzma),
        [0x1F, 0xA0, ..]
            => Compressed(Unsupported),       // LZH

        [.., 0x00,   _,    _ ] |
        [.., b' ', b' ', 0x00] if &magic_bytes[..5] == b"ustar"
            => Archive,

        _ => Unknown,
    })
}

fn untar<A: AsRef<Path>>(path: A) -> Result<Vec<PathBuf>, io::Error> {
    use TargetType::*;
    use CompressionType::*;

    match classify_target(path)? {
        Archive => {
            unimplemented!()
        },
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown archive format")),
    }
}