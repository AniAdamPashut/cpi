use serde::{Deserialize, Serialize};

use super::{ModuleError, Module};

#[derive(Deserialize)]
#[derive(Debug)]
pub struct PackageManifest {
    pub title: String,
    pub version: String,
    pub dependencies: Option<Vec<Dependency>>
}


#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dependency {
    pub name: String,
    pub version: String
}


impl Dependency {
    pub fn install(&self) -> Result<(), ModuleError> {
        Ok(
            Module::new(self.name.clone())?.install()?
        )
    }
}