// use std::fs;
use std::path::Path;
// use std::path::PathBuf;

pub const PATH: &str = "/opt/clibs";
// pub const METADATA_FOLDER: &str = "./.cpi/";
// const VERSION_PATH: &str = "version";

// pub fn read_lines_to_collection<T: FromIterator<String>>(filename: &str) -> Result<T, ModuleError> {
//     Ok(
//         fs::read_to_string(filename)?
//         .split("\n")
//         .map(|it: &str| it.to_owned())
//         .filter(|it: &String| !it.is_empty())
//         .collect()
//     )
// } // Maybe well need it later

// pub fn read_subdirs_to_collection<T: FromIterator<String>>(dirname: &str) -> Result<T, std::io::Error> {
//     fn is_directory(p: PathBuf) -> Option<PathBuf> {
//         if p.is_dir() {
//             return Some(p);
//         }
//         return None;
//     }
//     Ok(
//         fs::read_dir(dirname)?
//         .filter_map(
//             |it|
//             is_directory(
//                 it.expect("Failed to read entry")
//                 .path() 
//             )
//         )
//         .map(
//             |it| 
//             it.file_name().unwrap()
//             .to_owned()
//             .into_string().unwrap()
//         )
//         .filter(|it: &String| !it.is_empty())
//         .collect()
//     )
// } Maybe we will need it later

pub fn does_module_exist(path: &String) -> bool {
    Path::new(path).is_dir()
}
