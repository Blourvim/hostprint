#[derive(Debug, Clone)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
    pub family: String,
    pub kernel: String,
    pub arch: String,
}

impl OSInfo {
    pub fn detect(&self) -> OSInfo {
        self.detect_from_uname();
    }
    fn detect_from_uname() -> Option<OSInfo> {
    }
}
