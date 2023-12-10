use std::io::Error;
use std::collections::HashSet;
use crate::module::Module;
use crate::module::ModuleError;
use crate::utils::*;


#[derive(Debug)]
pub struct LocalModules {
    installed: HashSet<Module>
}

impl LocalModules {
    pub fn new() -> Result<LocalModules, ModuleError> {
        let installed: HashSet<Module> = read_lines_to_collection::<HashSet<String>>
            (INSTALLED_LIBRARIES_FILE)?
            .into_iter()
            .map(|it: String| Module::new(it).unwrap()) // TODO unwrap
            .collect();
        println!("{:?}", installed);
        Ok(LocalModules {
            installed: installed
        })
    }

    fn is_module_installed(&self, module: &Module) -> bool {
        return self.installed.contains(module);
    }
    pub fn install_module(&self, module: &Module) -> Result<(), Error> {
        if !self.is_module_installed(module) {
            println!("Installing {}", module.name);
            return module.install();
        }
        println!("The module {} is already installed", module.name);
        Ok(())
    }
}