use crate::module::Module;
use crate::module::ModuleError;
use crate::local::LocalModules;

mod version;
mod module;
mod utils;
mod local;

fn main() -> Result<(), ModuleError> {
    let mut local = LocalModules::new()?;
    let mut modu = Module::new(String::from("linkedlist"))?;
    let _ = local.install_module(modu.clone())?;
    modu.update(version::VersionOptions::Feature);
    println!("{:?}", modu);
    Ok(()) 
}
