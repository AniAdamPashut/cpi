use std::io::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use crate::utils::*;

#[derive(Debug)]
pub struct Module {
    pub dependencies: HashSet<Module>,
    pub name: String,
    pub path: String
}

impl Module {
    pub fn new(name: String) -> Result<Module, ModuleError> {
        let path = format!("{}/{}", PATH, name);
        let dependencies_path = format!("{}/dependencies", path);
        let dependencies: HashSet<Module> = read_subdirs_to_collection(dependencies_path.as_str())?;
        let module = Module {
            name: name,
            path: path,
            dependencies: dependencies
        };

        if does_module_exist(&module).is_err() {
            return Err(ModuleError::ModuleNotExist);
        }
        Ok(module)
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
}

impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

impl Eq for Module {}