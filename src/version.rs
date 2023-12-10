///
/// This Code goes by the Semantic Versioning Standard
/// https://semver.org/
/// 

mod version_struct;
mod version_options;
mod error;

pub use version_options::VersionOptions;
pub use error::VersionError;
pub use version_struct::Version;
