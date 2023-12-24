use crate::version::VersionError;

#[derive(Debug)]
pub enum ModuleError {
    IoError(std::io::Error),
    ModuleNotExist,
    BadVersion,
    MissingFile
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        return ModuleError::IoError(value);
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