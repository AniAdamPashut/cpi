use serde::Deserialize;

use super::Dependency;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct PackageManifest {
    pub title: String,
    pub version: String,
    pub dependencies: Option<Vec<Dependency>>
}
