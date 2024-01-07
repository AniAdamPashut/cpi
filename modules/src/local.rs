use std::{collections::HashSet, fs};
use serde::{Deserialize, Serialize};

use crate::module::Module;
use crate::prelude::ModuleError;
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
pub struct Local {
    package: Package,
    dependencies: Option<HashSet<Dependency>>
}


impl Local {
    pub fn is_installed(module: &Module) -> Result<bool, TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let manifest: Local = toml::from_str(&content)?;
        match manifest.dependencies {
            Some(dep) => Ok(dep.contains(&module.into())),
            None => Ok(false)
        }
    }

    pub fn add(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: Local = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.insert(module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }

    pub fn remove(module: &Module) -> Result<(), TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let mut manifest: Local = toml::from_str(&content)?;
        let mut dep = manifest.dependencies.unwrap_or(HashSet::new());
        dep.remove(&module.into());
        manifest.dependencies = Some(dep);
        let serialized = toml::to_string(&manifest)?;
        fs::write(METADATA_FILE, serialized)?;
        Ok(())
    }

    pub fn get_all() -> Result<String, TomlError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let manifest: Local = toml::from_str(&content)?;
        if manifest.dependencies.is_none() {
            return Ok(String::new());
        }
        let mut buffer = String::new();
        for dep in manifest.dependencies.unwrap() {
            buffer += format!("{}=={}", dep.name, dep.version).as_str();
        }
        Ok(buffer)
    }

    pub fn install_all() -> Result<(), ModuleError> {
        let content = fs::read_to_string(METADATA_FILE)?;
        let manifest: Local = toml::from_str(&content)?;
        if manifest.dependencies.is_none() {
            return Ok(())
        }
        for dep in manifest.dependencies.unwrap() {
            dep.install()?;
        }
        Ok(())
    }
}
