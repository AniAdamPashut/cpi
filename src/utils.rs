use std::fs;
use std::collections::HashSet;
use std::path::PathBuf;
use crate::module::Module;
use crate::module::ModuleError;
use crate::version::VersionError;
use crate::version::Version;

pub const PATH: &str = "/opt/clibs";
pub const METADATA_FOLDER: &str = "./.cpi/";
const VERSION_PATH: &str = "version";

// pub fn read_lines_to_collection<T: FromIterator<String>>(filename: &str) -> Result<T, ModuleError> {
//     Ok(
//         fs::read_to_string(filename)?
//         .split("\n")
//         .map(|it: &str| it.to_owned())
//         .filter(|it: &String| !it.is_empty())
//         .collect()
//     )
// } // Maybe well need it later

pub fn read_subdirs_to_collection<T: FromIterator<String>>(dirname: &str) -> Result<T, std::io::Error> {
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
            .split("/")
            .last()
            .unwrap_or("")
            .to_owned()
        )
        .filter(|it| !it.is_empty())
        .collect()
    )
}

pub fn does_module_exist(module: &Module) -> Result<(), ModuleError> {
    let modules: HashSet<String> = read_subdirs_to_collection(PATH)?;
    if modules.contains(&module.name) {
        return Ok(());
    } 
    return Err(ModuleError::ModuleNotExist);
}

pub fn get_version_of_module(path: &String) -> Result<Version, VersionError> {
    let file_path = format!("{path}/{}", VERSION_PATH);
    let content: Vec<i32> = 
        fs::read_to_string(file_path)?
        .trim()
        .split(".")
        .map(|it| it.parse::<i32>().unwrap_or(-1))
        .collect();
    if content.len() != 3 {
        return Err(VersionError::BadVersion)
    }
    return Ok(
        Version::new(
            content[0] as u32, 
            content[1] as u32, 
            content[2] as u32
        )
    )
}