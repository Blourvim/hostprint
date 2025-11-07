use std::collections::HashSet;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct SystemGroup {
    pub gid: Option<u32>,
    pub name: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemUser {
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub name: Option<String>,
    pub home: Option<String>,
    pub groups: Option<HashSet<SystemGroup>>,
}
