use crate::module::Module;
use crate::module::ModuleError;
use crate::local::LocalModules;
use crate::version::VersionOptions;

mod version;
mod module;
mod utils;
mod local;

fn main() -> Result<(), ModuleError> {
    let mut local = LocalModules::new()?;
    let mut modu = Module::new(String::from("linkedlist"))?;
    let _ = local.install_module(&modu)?;
    modu.update(VersionOptions::Feature);
    println!("{:?}", modu);
    Ok(()) 
}
