use crate::modules::Module;
mod modules;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    let modu = Module::new(String::from("linkedlist"))?;
    let _ = modu.install()?;
    Ok(())
}
