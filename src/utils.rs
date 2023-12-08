use std::fs;
use std::collections::HashSet;
use crate::module::Module;

pub const PATH: &str = "/opt/clibs";

pub const DEPENDENCIES_FILE: &str = "dependencies.txt";

pub const INSTALLED_LIBRARIES_FILE: &str = "./.cpi/installed.txt";

pub fn read_lines_to_collection<T: FromIterator<String>>(filename: &str) -> Result<T, std::io::Error> {
    Ok(
        fs::read_to_string(filename)?
        .split("\n")
        .map(|it: &str| it.to_owned())
        .filter(|it: &String| !it.is_empty())
        .collect()
    )
}

pub fn does_module_exist(module: &Module) -> bool {
    let modules = fs::read_dir(PATH)
    .expect("Failed to read directory")
    .map(|entry| entry.expect("Failed to read entry").path().to_str().unwrap().to_owned())
    .collect::<HashSet<_>>();
    return modules.contains(&module.path)
}