use std::io::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

use crate::utils::*;
use crate::version::{Version, VersionOptions};

use super::toml::{PackageManifest, Dependency};
use super::ModuleError;

#[derive(Debug, Clone)]
pub struct Module {
    pub dependencies: HashSet<Dependency>,
    pub version: Version,
    pub name: String,
    pub path: String
}

impl Module {
    pub fn new(name: String) -> Result<Module, ModuleError> {
        let path = format!("{}/{}", PATH, name);
        if !does_module_exist(&path) {
            return Err(ModuleError::ModuleNotExist);
        }
        let path_to_toml = format!("{}/metadata.toml", path);
        let manifest_content = fs::read_to_string(path_to_toml)?;
        let pkg: PackageManifest = toml::from_str(&manifest_content).unwrap();
        Ok(
            Module {
                    dependencies: HashSet::from_iter(pkg.dependencies.unwrap_or(Vec::new()).into_iter()),
                    version: Version::try_from(pkg.version)?,
                    name: pkg.title,
                    path: path
            }
        )
    }

    pub fn install(&self) -> Result<(), Error> {
        let _ = self.install_dependencies()?;

        println!("Downloading {}", &self.name);
        let header = format!("{}/{}.h", self.path, self.name);
        let _ = fs::copy(
            header, 
            format!("./build/libs/headers/{}.h", self.name)
        )?;
        let shared = format!("{}/{}.so", self.path, self. name);
        let _ = fs::copy(
            shared, 
            format!("./build/libs/objs/{}.so", self.name)
        )?;

        Ok(())
    }
    
    fn install_dependencies(&self) -> Result<(), Error>{
        if self.dependencies.is_empty() {
            return Ok(());
        }
        self.dependencies.iter().for_each(|f| f.install().unwrap());
        Ok(())
    }

    pub fn update(&mut self, opt: VersionOptions) {
        self.version.increment(opt);
    }
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name && self.version == other.version;
    }
}

impl Eq for Module {}