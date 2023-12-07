use std::io::Error;
use std::fs;

#[derive(Debug)]
pub struct Module {
    pub dependencies: Vec<String>,
    pub name: String,
    pub path: String
}

impl Module {
    pub fn new(name: String) -> Result<Module, Error> {
        let path = format!("{}/{}", super::PATH, name);
        let dependencies: Vec<String> = 
            fs::read_to_string(
                format!("{}/dependencies.txt", path)
            )?
            .split("\n")
            .map(|it: &str| it.to_owned())
            .filter(|it: &String| !it.is_empty())
            .collect();
        Ok(Module {
            name: name,
            path: path,
            dependencies: dependencies
        })
    }
    pub fn install(&self) -> Result<(), Error> {
        let header = format!("{}/{}.h", self.path, self.name);
        let _ = fs::copy(
            header, 
            format!("./build/libs/headers/{}.h", self.name)
        )?;
        let shared = format!("{}/{}.so", self.path, self. name);
        let _ = fs::copy(
            shared, 
            format!("./build/libs/headers/{}.so", self.name)
        )?;

        let _ = self.install_dependencies();
        Ok(())
    }
    
    pub fn install_dependencies(&self) -> Result<(), Error>{
        if self.dependencies.is_empty() {
            return Ok(());
        }
        for dependency in &self.dependencies {
            let dependency_module = Module::new(dependency.to_owned())?;
            dependency_module.install()?;
        }
        Ok(())
    }
}