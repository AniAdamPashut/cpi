use clap::Parser;

use modules::prelude::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The operation to have (install/uninstall)
    flags: String,

    /// The Module to install
    module: Option<String>,
}

fn main() -> Result<(), ModuleError> {
    let args = Args::parse();
    
    if args.flags == "freeze" {
        println!("{}", Local::get_all()?);
        return Ok(());
    }

    let flags: &[char] = &args
    .flags
    .chars()
    .collect::<Vec<char>>();
    let module = Module::new(args.module.unwrap_or("".to_owned()))?;
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

