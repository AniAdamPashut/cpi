use std::fs;
use std::collections::HashSet;
use std::path::PathBuf;
use crate::module::Module;
use crate::module::ModuleError;

pub const PATH: &str = "/opt/clibs";
pub const INSTALLED_LIBRARIES_FILE: &str = "./.cpi/installed.txt";

pub fn read_lines_to_collection<T: FromIterator<String>>(filename: &str) -> Result<T, ModuleError> {
    Ok(
        fs::read_to_string(filename)?
        .split("\n")
        .map(|it: &str| it.to_owned())
        .filter(|it: &String| !it.is_empty())
        .collect()
    )
}

pub fn read_subdirs_to_collection<T: FromIterator<Module>>(dirname: &str) -> Result<T, std::io::Error> {
    fn is_directory(p: PathBuf) -> Option<PathBuf> {
        if p.is_dir() {
            return Some(p);
        }
        return None;
    }
    Ok(
        fs::read_dir(dirname)?
        .filter_map(
            |it|
            is_directory(
                it.expect("Failed to read entry")
                .path() 
            )
        )
        .map(
            |it| 
            it.to_string_lossy()
            .to_string()
        )
        .filter(|it| !it.is_empty())
        .map(|it| Module::new(it).unwrap())
        .collect()
    )
}

pub fn does_module_exist(module: &Module) -> Result<(), ModuleError> {
    let modules: HashSet<Module> = read_subdirs_to_collection(PATH)?;
    if modules.contains(module) {
        return Ok(());
    } 
    return Err(ModuleError::ModuleNotExist);
}