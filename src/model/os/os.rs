#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct OSInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub family: Option<String>,
    pub kernel: Option<String>,
    pub arch: Option<String>,
}
