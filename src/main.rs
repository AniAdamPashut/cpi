use crate::module::Module;
use crate::module::ModuleError;

mod version;
mod module;
mod local;
mod toml;


fn main() -> Result<(), ModuleError> {
    let modu = Module::new(String::from("linkedlist"))?;
    modu.install()?;
    Ok(())
}
