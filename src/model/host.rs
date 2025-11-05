use crate::model::hardware::hardware::Hardware;
use std::{collections::HashSet, default, u64};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
enum Os {
    Arch(String),
    Debian(String),
    Other(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
enum PackageManagers {
    Pacman(String),
    Apt(String),
    Pamac(String),
    Flatpak(String),
    Other(String),
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Hash, PartialEq, Eq)]
struct Group {
    pid: Option<u64>,
    name: Option<u64>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
struct SystemUser {
    pid: Option<u64>,
    groups: Option<HashSet<Group>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Host {
    // current_user: SystemUser,
    os: Option<Os>,
    hardware: Option<Hardware>,
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
