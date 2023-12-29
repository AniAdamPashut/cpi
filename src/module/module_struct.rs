use std::io::Error;
use std::fs;
use std::collections::HashSet;
use std::path::{PathBuf, Path};

use crate::local::LocalManifest;
use crate::version::Version;

use super::toml::{PackageManifest, Dependency};
use super::ModuleError;

const PATH: &str = "/opt/clibs";

#[derive(Debug, Clone)]
pub struct Module {
    pub dependencies: HashSet<Dependency>,
    pub version: Version,
    pub name: String,
    pub path: PathBuf
}

impl Module {
    pub fn new(name: String) -> Result<Module, ModuleError> {
        let path: PathBuf = format!("{}/{}", PATH, name).into();
        if !path.is_dir() {
            return Err(ModuleError::ModuleNotExist);
        }
        let path_to_toml = format!("{}/metadata.toml", path.display());
        let manifest_content = fs::read_to_string(path_to_toml)?;
        let pkg: PackageManifest = toml::from_str(&manifest_content).unwrap();
        Ok(
            Module {
                    dependencies: HashSet::from_iter(pkg.dependencies.unwrap_or(Vec::new()).into_iter()),
                    version: Version::try_from(pkg.version)?,
                    name: pkg.title,
                    path: path
            }
        )
    }

    pub fn install(&self) -> Result<(), ModuleError> {
        self.install_dependencies()?;

        match LocalManifest::is_installed(self) {
            Ok(boolean) => {
                if boolean {
                    println!("Installed already");
                    return Ok(());
                }
            }
            Err(e) => return Err(e.into())
        }

        println!("Downloading {}", &self.name);
        let header = format!("{}/{}.h", self.path.display(), self.name);
        fs::copy(
            header, 
            format!("./build/libs/headers/{}.h", self.name)
        )?;
        let shared = format!("{}/{}.so", self.path.display(), self. name);
        fs::copy(
            shared, 
            format!("./build/libs/objs/{}.so", self.name)
        )?;

        match LocalManifest::update_in_manifest(self) {
            Ok(_) => Ok(()),
            Err(e) => {
                self.uninstall();
                Err(e.into())
            }
        }
    }


    /// # Panics
    /// Panics if the wanted files has open file descriptors
    pub fn uninstall(&self) {
        fs::remove_file(Path::new(&format!("./build/libs/objs/{}.so", self.name))).unwrap();
        fs::remove_file(Path::new(&format!("./build/libs/headers/{}.h", self.name))).unwrap();
    }
    
    fn install_dependencies(&self) -> Result<(), Error>{
        self.dependencies.iter().for_each(|f| f.install().unwrap());
        Ok(())
    }
}

impl From<&Module> for Dependency {
    fn from(module: &Module) -> Dependency {
        Dependency {
            name: module.name.to_owned(),
            version: module.version.into()
        }
    }
}

impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name && self.version == other.version;
    }
}

impl Eq for Module {}