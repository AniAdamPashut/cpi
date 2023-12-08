use crate::module::Module;
mod module;
mod utils;

fn main() -> Result<(), std::io::Error> {
    let modu = Module::new(String::from("linkedlist"))?;
    let _ = modu.install()?;
    Ok(())
}
