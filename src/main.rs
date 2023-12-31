use clap::Parser;
use crate::module::Module;
use crate::module::ModuleError;

mod version;
mod module;
mod local;
mod toml;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The operation to have (install/uninstall)
    flags: String,

    /// The Module to install
    module: String,
}

fn main() -> Result<(), ModuleError> {
    let args = Args::parse();
    let module = Module::new(args.module)?;
    let flags: &[char] = &args
                            .flags
                            .chars()
                            .collect::<Vec<char>>();
    match flags {
        ['S', 'y', 'u'] => {
            println!("Upgrading all");
        }
        ['S', ..] => { // Todo ['S', tail @ ..]
            module.install()?;
            println!("Installed: {}", &module.name);
        }
        ['R', ..] => { // Todo ['R', tail @ ..]
            module.uninstall()?;
            println!("Removed: {}", &module.name);
        }
        _ => println!("Unknown set of flags {}", args.flags)
    }
    Ok(())
}

