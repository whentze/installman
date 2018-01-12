#![feature(slice_patterns, advanced_slice_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;
extern crate tar;

mod config;
pub mod error;

pub mod lib {
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::io::{self, Read};
    use std::fmt;
    use ::std::ffi::OsStr;

    use error::*;

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
            use self::CompressionType::*;
            use self::TargetType::*;
            match *self {
                Executable(_) => write!(f, "Executable Binary"),
                Directory => write!(f, "Directory"),
                Archive => write!(f, "Archive"),
                Compressed(Gzip) => write!(f, "Compressed Gzip"),
                Compressed(Lzw) => write!(f, "Compressed Lzw"),
                Compressed(Lzma) => write!(f, "Compressed Lzma"),
                Compressed(Bzip2) => write!(f, "Compressed Bzip"),
                Compressed(Unsupported) => write!(f, "Compressed Unsupported"),
                Unknown => write!(f, "Unknown Target Type"),
            }
        }
    }

    pub fn classify_target<A: AsRef<Path>>(path: A) -> Result<TargetType> {
        use self::TargetType::*;
        use self::ExecutableType::*;
        use self::CompressionType::*;

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

            [.., 0x00, _, _ ] |
            [.., b' ', b' ', 0x00] if &magic_bytes[..5] == b"ustar"
            => Archive,

            _ => Unknown,
        })
    }

    pub fn init() -> Result<()> {
        use std::fs;
        use config::Config;

        let config = Config::default();
        config.init()?;
        fs::File::create(&*super::config::DATA_LOCATION)?;

        fs::create_dir_all(&*super::config::APPS_LOCATION)?;
        fs::create_dir_all(&*super::config::DESKTOP_FILES_LOCATION)?;
        fs::create_dir_all(&*super::config::BIN_SYMLINK_LOCATION)?;
        Ok(())
    }

    fn untar<A: AsRef<Path>>(path: A) -> Result<Vec<PathBuf>> {
        use self::TargetType::*;
        use self::CompressionType::*;

        match classify_target(path)? {
            Archive => {
                unimplemented!()
            },
            _ => Err(err_msg("Not a recognized archive format.")),
        }
    }

    fn add_symlink<A: AsRef<Path>>(dest: A, symlink_name: &str) -> Result<()> {
        use config::Config;
        use std::os::unix::fs;

        let mut path = super::config::BIN_SYMLINK_LOCATION.to_path_buf();
        path.push(symlink_name);
        fs::symlink(dest, path);
        Ok(())
    }

    fn get_app_name<A: AsRef<Path>>(path_app: A) -> Result<String> {
        use std::path::Path;
        use self::TargetType::*;
        use self::ExecutableType::*;

        Ok(match classify_target(&path_app)? {
            Executable(_) => Path::file_stem(path_app.as_ref()).unwrap().to_string_lossy().into_owned(),
            _ => "appname_dummy".to_string()
        })
    }

    pub fn install_target<A: AsRef<Path>>(path: A) -> Result<(String)> {
        use self::TargetType::*;
        use self::ExecutableType::*;
        match classify_target(&path)? {
            Executable(_) => Ok(install_executable(&path)?),
            _ => Err(err_msg("Installation not possible")),
        }
    }

    fn install_executable<A: AsRef<Path>>(path_exec: A) -> Result<(String)> {
        use config::Config;
        use std::fs::copy;

        let app_name = get_app_name(&path_exec)?;
        let mut dest_path = super::config::APPS_LOCATION.to_path_buf();
        dest_path.push(&app_name);
        fs::create_dir_all(&dest_path)?;
        dest_path.push(&*path_exec.as_ref().file_name().unwrap());
        copy(path_exec, &dest_path);
        add_symlink(&dest_path, &app_name);
        Ok((app_name))
    }
}