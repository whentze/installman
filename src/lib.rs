#![feature(slice_patterns, advanced_slice_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate toml;
extern crate tar;

pub mod config;
pub mod error;

pub mod lib {
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::io::{Read, Write};
    use std::fmt;
    use error::*;
    use config::{CONFIG, DATA, Data};

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

        fs::create_dir_all(&CONFIG.read().unwrap().apps_location)?;
        fs::create_dir_all(&CONFIG.read().unwrap().desktop_files_location)?;
        fs::create_dir_all(&CONFIG.read().unwrap().bin_symlink_location)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn untar<A: AsRef<Path>>(path: A) -> Result<Vec<PathBuf>> {
        use self::TargetType::*;

        match classify_target(path)? {
            Archive => {
                unimplemented!()
            },
            _ => Err(ErrorKind::UnrecognizedArchiveFormat.into()),
        }
    }

    fn create_symlink<A: AsRef<Path>>(dest: A, symlink_name: &str) -> Result<()> {
        use std::os::unix::fs;

        let mut path = (&CONFIG.read().unwrap().bin_symlink_location).clone();
        path.push(symlink_name);
        fs::symlink(dest, path)?;
        Ok(())
    }

    fn remove_symlink (name: &str) -> Result<()>{
        use std::fs;

        let mut path = (&CONFIG.read().unwrap().bin_symlink_location).clone();
        path.push(name);
        fs::remove_file(path);
        Ok(())
    }


    pub fn get_app_name<A: AsRef<Path>>(path_app: A) -> Result<String> {
        use std::path::Path;
        use self::TargetType::*;

        Ok(match classify_target(&path_app)? {
            Executable(_) => Path::file_stem(path_app.as_ref()).unwrap().to_string_lossy().into_owned(),
            _ => "appname_dummy".to_string()
        })
    }

    pub fn install_target<A: AsRef<Path>>(path: A, name: String) -> Result<String> {
        use self::TargetType::*;
        println!("install_target: path: {:?}, name: {:?}", path.as_ref().to_str(), name);
        match classify_target(&path)? {
            Executable(_) => {
                Ok(install_executable(&path, name)?)
            },
            _ => {
                println!("scuuuur");
                Err(ErrorKind::TargetTypeNotSupported.into())
            },
        }
    }

    pub fn uninstall_target(name: &str) -> Result<()>{
        remove_symlink(name);
        remove_desktop_file(name);
        remove_app(name);
        remove_data(name);
        Ok(())
    }

    fn install_executable<A: AsRef<Path>>(path_exec: A, app_name: String) -> Result<String> {
        use std::fs::copy;

        if app_exists(&app_name) {
            return Err(ErrorKind::AlreadyInstalledApp(app_name).into());
        }
        let mut dest_path = (&CONFIG.read().unwrap().apps_location).clone();
        dest_path.push(&app_name);

        fs::create_dir_all(&dest_path)?;
        dest_path.push(&*path_exec.as_ref().file_name().unwrap());
        copy(path_exec, &dest_path)?;
        create_symlink(&dest_path, &app_name);
        create_desktop_file(&app_name);

        let new_app = super::config::App{name: app_name.clone()};
        DATA.write().unwrap().installed_apps.push(new_app);
        Data::store()?;
        Ok((app_name))
    }

    fn app_exists(name: &str) -> bool{
        DATA.read().unwrap().installed_apps.iter().any(|x| x.name == name)
    }

    fn create_desktop_file(name: &str) -> Result<()> {
        let content = format!("\
            [Desktop Entry]\n\
            Type=Application\n\
            Version=1.1\n\
            Name={name}\n\
            Path={path}\n\
            Comment=Brought to you by installman!\n\
            Exec={exec}\n\
            TryExec={exec}",
            name=name,
            path=&CONFIG.read().unwrap().apps_location.join(name).to_string_lossy(),
            exec=&CONFIG.read().unwrap().bin_symlink_location.join(name).to_string_lossy());
        let mut file = File::create(&CONFIG.read().unwrap()
            .desktop_files_location
            .join(name)
            .with_extension("desktop"))?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn remove_desktop_file(name: &str) -> Result<()>{
        use std::fs;

        let mut path = (&CONFIG.read().unwrap().desktop_files_location).clone();
        path.push(name);
        path.set_extension("desktop");
        println!("{:?}", path);
        fs::remove_file(path);
        Ok(())
    }

    fn remove_app(name: &str) -> Result<()> {
        use std::fs;

        let mut path = (&CONFIG.read().unwrap().apps_location).clone();
        path.push(name);
        fs::remove_dir_all(path);
        Ok(())
    }

    fn remove_data (name: &str) -> Result<()>{
        DATA.write().unwrap().installed_apps.retain(|app| app.name != name);
        Data::store();
        Ok(())
    }
}