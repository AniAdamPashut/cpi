use crate::{version::VersionError, toml::TomlError};

#[derive(Debug)]
pub enum ModuleError {
    IoError(std::io::Error),
    DeserializationError(toml::de::Error),
    SerializationError(toml::ser::Error),
    ModuleNotExist,
    BadVersion
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        return ModuleError::IoError(value);
    }
}

impl From<toml::de::Error> for ModuleError {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value)
    }
}

impl From<TomlError> for ModuleError {
    fn from(value: TomlError) -> Self {
        match value {
            TomlError::IoError(io) => ModuleError::IoError(io),
            TomlError::DeserializationError(err) => ModuleError::DeserializationError(err),
            TomlError::SerializationError(err) => ModuleError::SerializationError(err),
        }
    }
}

impl From<VersionError> for ModuleError {
    fn from(value: VersionError) -> Self {
        match value {
            VersionError::BadVersion => return ModuleError::BadVersion,
            VersionError::Io(io_error) => return ModuleError::IoError(io_error)
        }
    }
}