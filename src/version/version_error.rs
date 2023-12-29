#[derive(Debug)]
pub enum VersionError {
    Io(std::io::Error),
    BadVersion
}

impl From<std::io::Error> for VersionError {
    fn from(value: std::io::Error) -> Self {
        return VersionError::Io(value);
    }
}