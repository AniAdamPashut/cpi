use super::VersionOptions;

#[derive(Debug, Clone, Copy)]
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