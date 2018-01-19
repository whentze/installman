error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Toml(::toml::ser::Error);
    }
    errors {
        AlreadyInstalledApp(n: String) {
            description("App already installed"),
            display("The app {} is already installed.", n),
        }
        UnrecognizedArchiveFormat {
            description("Unrecognized archive format")
            display("The archive is not of a supported format.")
        }
        TargetTypeNotSupported {
            description("Unsupported target type")
            display("Support for installing apps from this type of file has not yet been implemented.")
        }
    }
}

pub use self::ErrorKind::*;