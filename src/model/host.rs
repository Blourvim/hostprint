use crate::model::{hardware::hardware::Hardware, os::os::OSInfo};
use std::{collections::HashSet, default, u64};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Os {
    Arch(String),
    Debian(String),
    Other(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum PackageManagers {
    Pacman(String),
    Apt(String),
    Pamac(String),
    Flatpak(String),
    Other(String),
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Group {
    pub pid: Option<u64>,
    pub name: Option<u64>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SystemUser {
    pub pid: Option<u64>,
    pub groups: Option<HashSet<Group>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Host {
    // current_user: SystemUser,
    pub os: Option<OSInfo>,
    pub hardware: Option<Hardware>,
    // package_managers: HashSet<PackageManagers>,
}

impl Default for Host {
    fn default() -> Host {
        Host {
            os: None,
            hardware: None,
        }
    }
}

impl Host {
    pub fn new() -> Host {
        Host::default()
    }
}
