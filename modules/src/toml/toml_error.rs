
#[derive(Debug)]
pub enum TomlError {
    IoError(std::io::Error),
    DeserializationError(toml::de::Error),
    SerializationError(toml::ser::Error),
}

impl From<std::io::Error> for TomlError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for TomlError {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value)
    }
}

impl From<toml::ser::Error> for TomlError {
    fn from(value: toml::ser::Error) -> Self {
        Self::SerializationError(value)
    }
}