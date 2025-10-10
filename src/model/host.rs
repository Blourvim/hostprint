use crate::model::hardware::hardware::Hardware;
use std::{collections::HashSet, default, u64};

enum Os {
    Arch(String),
    Debian(String),
    Other(String),
}

enum PackageManagers {
    Pacman(String),
    Apt(String),
    Pamac(String),
    Flatpak(String),
    Other(String),
}
struct Group {
    pid: Option<u64>,
    name: Option<u64>,
}
struct SystemUser {
    pid: Option<u64>,
    groups: Option<HashSet<Group>>,
}

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
    fn new() -> Host {
        Host::default()
    }
}
