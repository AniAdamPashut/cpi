use std::{collections::HashSet, fs};
use serde::{Deserialize, Serialize};

use crate::module::{Dependency, Module};

const METADATA_FILE: &str = "./cpi.toml";

#[derive(Deserialize, Serialize)]
pub struct LocalManifest {
    dependencies: Option<HashSet<Dependency>>
}

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

impl LocalManifest {
    pub fn is_installed(module: &Module) -> Result<bool, TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let manifest: LocalManifest = toml::from_str(&content)?;
        match manifest.dependencies {
            Some(dep) => Ok(dep.contains(&module.into())),
            None => Ok(false)
        }
    }

    pub fn update_in_manifest(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: LocalManifest = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.insert(module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }
}
