use crate::module::Module;
use crate::module::PackageManifest;
use crate::module::ModuleError;
use crate::local::LocalModules;
use crate::version::VersionOptions;

use std::fs::read_to_string;

mod version;
mod module;
mod utils;
mod local;

fn main() -> Result<(), ModuleError> {
    let path = "/opt/clibs/linkedlist/metadata.toml";
    let contents = read_to_string(path)?;
    let manifest: Result<PackageManifest, toml::de::Error> = toml::from_str(&contents);
    match manifest {
        Ok(c) => println!("{:?}", c),
        Err(e) => eprintln!("{}", e)
    }
    Ok(())
}
