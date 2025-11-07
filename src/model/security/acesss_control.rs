use std::collections::HashSet;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct SystemGroup {
    pub pid: Option<u64>,
    pub name: Option<u64>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemUser {
    pub pid: Option<u64>,
    pub groups: Option<HashSet<SystemGroup>>,
}
