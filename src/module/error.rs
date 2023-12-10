#[derive(Debug)]
pub enum ModuleError {
    IoError(std::io::Error),
    ModuleNotExist
}

impl From<std::io::Error> for ModuleError {
    fn from(value: std::io::Error) -> Self {
        return ModuleError::IoError(value);
    }
}
