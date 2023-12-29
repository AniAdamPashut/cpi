use std::{collections::HashSet, fs};
use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::toml::{Dependency, TomlError};

const METADATA_FILE: &str = "./cpi.toml";

#[derive(Deserialize, Serialize)]
pub struct LocalManifest {
    dependencies: Option<HashSet<Dependency>>
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

    pub fn add(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: LocalManifest = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.insert(module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }

    pub fn remove(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: LocalManifest = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.remove(&module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }
}
