use std::io::{Error, Write};
use std::fs::{self, File};
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
                    "Module doesn't exist in db3"
                )
            )
        }
        Ok(module)
    }

    pub fn is_installed(&self) -> bool {
        match read_lines_to_collection::<HashSet<String>>(INSTALLED_LIBRARIES_FILE) {
            Ok(installed_modules) => return installed_modules.contains(&self.name),
            Err(_) => return false
        }
    }

    pub fn install(&self) -> Result<(), Error> {
        if self.is_installed() {
            println!("Already installed");
            return Ok(())
        }

        let _ = self.install_dependencies()?;

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
        
        self.update_in_file();

        Ok(())
    }

    fn update_in_file(&self) {
        let mut f = File::options()
        .append(true)
        .open(INSTALLED_LIBRARIES_FILE)
        .expect("Like how tf we got here??? (dont have a .cpi directory)");
        writeln!(&mut f, "{}", self.name.as_str()).expect("Ok now im really confused");
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