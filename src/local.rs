use std::io::Error;
use std::collections::HashSet;
use crate::module::{Module, ModuleError};
use crate::utils::*;


#[derive(Debug)]
pub struct LocalModules {
    installed: HashSet<Module>
}

impl LocalModules {
    pub fn new() -> Result<LocalModules, ModuleError> {
        let installed: HashSet<Module> = read_subdirs_to_collection::<HashSet<String>>
            (METADATA_FOLDER)?
            .into_iter()
            .map(|it| Module::new(it).unwrap())
            .collect();
        println!("{:?}", installed);
        Ok(LocalModules {
            installed: installed
        })
    }

    fn is_module_installed(&self, module: &Module) -> bool {
        return self.installed.contains(module);
    }
    pub fn install_module(&mut self, module: &Module) -> Result<(), Error> {
        if !self.is_module_installed(module) {
            println!("Installing {}", module.name);
            match module.install() {
                Ok(()) => {
                    self.installed.insert(module.to_owned());
                    return Ok(());
                },
                Err(e) => return Err(e)
            }
        }
        println!("The module {} is already installed", module.name);
        Ok(())
    }
}