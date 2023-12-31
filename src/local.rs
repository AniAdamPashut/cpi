use std::{collections::HashSet, fs};
use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::toml::{Dependency, TomlError};

const METADATA_FILE: &str = "./cpi.toml";

///
/// ## This struct is only here so serde wouldnt override these values when serializing
/// 
#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    author: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct LocalLibraries {
    package: Package,
    dependencies: Option<HashSet<Dependency>>
}


impl LocalLibraries {
    pub fn is_installed(module: &Module) -> Result<bool, TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let manifest: LocalLibraries = toml::from_str(&content)?;
        match manifest.dependencies {
            Some(dep) => Ok(dep.contains(&module.into())),
            None => Ok(false)
        }
    }

    pub fn add(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: LocalLibraries = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.insert(module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }

    pub fn remove(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: LocalLibraries = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.remove(&module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }
}
