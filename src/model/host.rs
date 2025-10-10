use std::collections::HashSet;

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
    Other(String)
}

pub struct Host {
    os: Option<Os>,
    memory_total:u64,
    memory_used:u64,
    package_managers:HashSet<PackageManagers>,

}
