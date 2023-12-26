use crate::module::Module;
use crate::module::ModuleError;

mod version;
mod module;
mod utils;
mod local;

fn main() -> Result<(), ModuleError> {
    let modu = Module::new(String::from("linkedlist"))?;
    modu.install()?;
    Ok(())
}
