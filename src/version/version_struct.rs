use core::num;

use super::VersionOptions;
use super::VersionError;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32
}


impl Default for Version {
    fn default() -> Self {
        Version { major: 0, minor: 1, patch: 0 }
    }
}

impl Version {
    pub fn new(
        major: u32,
        minor: u32,
        patch: u32
    ) -> Version {
        Version {
            major: major,
            minor: minor,
            patch: patch
        }
    }

    pub fn increment(&mut self, opt: VersionOptions) {
        match opt {
            VersionOptions::Bugfix => self.patch += 1,
            VersionOptions::Feature => {
                self.minor += 1;
                self.patch = 0;
            }
            VersionOptions::Release => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
        }
    }
}


impl TryFrom<String> for Version {
    type Error = VersionError;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        let numbers: Vec<u32> = name
        .split(".")
        .into_iter()
        .map(|it| it.parse::<i32>().unwrap_or(-1))
        .filter_map(|it| it.try_into().ok())
        .collect();
        if numbers.len() != 3 {
            return Err(VersionError::BadVersion);
        }
        Ok(
            Version {
                major: numbers[0],
                minor: numbers[1],
                patch: numbers[2]
            }
        )

    }
}