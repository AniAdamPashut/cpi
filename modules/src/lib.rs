mod module;
mod toml;
mod version;
mod local;
pub mod prelude;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, collections::HashSet};

    use crate::{module::Module, version::Version};
    
    #[test]
    fn module() {
        let module = Module::new(String::from("linkedlist"));
        assert!(module.is_ok());
        let module = module.unwrap();
        assert_eq!(module.name, String::from("linkedlist"));
        assert_eq!(module.path, PathBuf::from("/opt/clibs/linkedlist"));
        assert_eq!(module.version, Version::new(0, 1, 0));
        assert_eq!(module.dependencies, HashSet::new());
    }

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
