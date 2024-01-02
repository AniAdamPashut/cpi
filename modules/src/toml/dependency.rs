use serde::{Deserialize, Serialize};
use crate::module::{Module, ModuleError};


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