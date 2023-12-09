use crate::module::Module;
use crate::local::LocalModules;

mod module;
mod utils;
mod local;

fn main() -> Result<(), utils::ModuleError> {
    let local = LocalModules::new()?;
    let modu = Module::new(String::from("linkedlist"))?;
    let _ = local.install_module(&modu)?;
    Ok(())
}
