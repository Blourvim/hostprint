use crate::model::{
    facts::{package_manager::PackageManagerFacts, packages::PackagesFacts},
    host::{Host, PackageManager},
};
use std::collections::HashSet;

pub fn package_manager_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = PackageManagerFacts::from_str(stdout);
    if let Some(path) = facts.found_path {
        if path.contains("apt") {
            host.package_managers.insert(PackageManager::Apt(path));
        } else if path.contains("dpkg") {
            // dpkg is usually present with apt, but we can track it
             host.package_managers.insert(PackageManager::Other("dpkg".to_string()));
        } else if path.contains("yum") {
             host.package_managers.insert(PackageManager::Other("yum".to_string()));
        } else if path.contains("dnf") {
             host.package_managers.insert(PackageManager::Other("dnf".to_string()));
        } else if path.contains("pacman") {
             host.package_managers.insert(PackageManager::Pacman(path));
        } else if path.contains("rpm") {
             host.package_managers.insert(PackageManager::Other("rpm".to_string()));
        }
    }
}

pub fn packages_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = PackagesFacts::from_dpkg(stdout);
    if let Some(existing) = &mut host.packages {
        existing.extend(facts.packages);
    } else {
        host.packages = Some(facts.packages);
    }
}
