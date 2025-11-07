use crate::model::{
    hardware::hardware::Hardware,
    metrics::metrics::Metrics,
    os::os::OSInfo,
    security::{
        acesss_control::{SystemGroup, SystemUser},
        session::ActiveSession,
    },
};
use std::{collections::HashSet, u64};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Os {
    Arch(String),
    Debian(String),
    Other(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PackageManager {
    Pacman(String),
    Apt(String),
    Pamac(String),
    Flatpak(String),
    Other(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Host {
    pub current_user: Option<SystemUser>,
    pub os: Option<OSInfo>,
    pub hardware: Option<Hardware>,
    pub users: Option<Vec<SystemUser>>,
    pub groups: Option<Vec<SystemGroup>>,
    pub package_managers: HashSet<PackageManager>,
    pub metrics: Option<Metrics>,
    pub sessions: Option<Vec<ActiveSession>>,
}

impl Default for Host {
    fn default() -> Host {
        Host {
            package_managers: HashSet::new(),
            users: None,
            groups: None,
            current_user: None,
            os: None,
            hardware: None,
            metrics: None,
            sessions: None,
        }
    }
}

impl Host {
    pub fn new() -> Host {
        Host::default()
    }
}
