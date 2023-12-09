use std::io::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use crate::utils::*;

#[derive(Debug)]
pub struct Module {
    pub dependencies: HashSet<String>,
    pub name: String,
    pub path: String
}

impl Module {
    pub fn new(name: String) -> Result<Module, Error> {
        let path = format!("{}/{}", PATH, name);
        let path_to_file = format!("{path}/{}", DEPENDENCIES_FILE);
        let dependencies: HashSet<String> = read_lines_to_collection(path_to_file.as_str())?;
        let module = Module {
            name: name,
            path: path,
            dependencies: dependencies
        };

        if !does_module_exist(&module) {
            return Err(
                Error::new(
                    std::io::ErrorKind::NotFound, 
                    "Module doesn't exist in db"
                )
            )
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
        for dependency in &self.dependencies {
            let dependency_module = Module::new(dependency.to_owned())?;
            dependency_module.install()?;
        }
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