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
}

impl From<Version> for String {
    fn from(version: Version) -> String {
        format!("{}.{}.{}", version.major, version.minor, version.patch)
    }
}



impl TryFrom<String> for Version {
    type Error = VersionError;
    fn try_from(version: String) -> Result<Self, Self::Error> {
        let numbers: Vec<u32> = version
        .split(".")
        .filter_map(|it| it.parse::<u32>().ok())
        .collect();
        if numbers.len() != 3 {
            return Err(VersionError::BadVersion);
        }
        let [maj, min, pat] = numbers[..] else {panic!("if we got here we screwed up real bad")};
        // for the panic 
        /* the vector should always has length of 3
           if it has any length other then 3 it would trigger the panic.
         */
        Ok(
            Version::new(maj, min, pat)
        )

    }
}