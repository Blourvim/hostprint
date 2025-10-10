#[derive(Debug, Clone)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
    pub family: String,
    pub kernel: String,
    pub arch: String,
}

impl OSInfo {
    fn detect_from_uname() -> Option<OSInfo> {
        const COMMAND: String = "uname -a".into();
    }
}
