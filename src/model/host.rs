use crate::model::{
    facts::id::Group,
    hardware::hardware::Hardware,
    os::os::OSInfo,
    security::acesss_control::{SystemGroup, SystemUser},
};
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Host {
    current_user: Option<SystemUser>,
    pub os: Option<OSInfo>,
    pub hardware: Option<Hardware>,
    pub users: Option<Vec<SystemUser>>,
    pub groups: Option<Vec<SystemGroup>>,
    // package_managers: HashSet<PackageManagers>,
}

impl Default for Host {
    fn default() -> Host {
        Host {
            users: None,
            groups: None,
            current_user: None,
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
