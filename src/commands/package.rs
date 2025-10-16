use crate::{commands::unit::Unit, model::host::Host};

fn noop_follow_up(stdout: &str, stderr: &str, _host: &mut Host) {
    println!("Command output:\n{}", stdout);
    if !stderr.trim().is_empty() {
        eprintln!("stderr: {}", stderr);
    }
}

pub fn package_units() -> Vec<Unit> {
    vec![
        // Detect package manager
        Unit::new("Package Manager", "which apt || which dpkg || which yum || which dnf || which rpm || which pacman", noop_follow_up),

        // Debian/Ubuntu
        Unit::new("Installed Packages (dpkg)", "dpkg -l | head -20", noop_follow_up),
        Unit::new("Installed Packages Count (dpkg)", "dpkg -l | wc -l", noop_follow_up),
        Unit::new("APT Updates Available", "apt list --upgradable 2>/dev/null | head -20", noop_follow_up),

        // RedHat/CentOS/Fedora
        Unit::new("Installed Packages (rpm)", "rpm -qa | head -20", noop_follow_up),
        Unit::new("Installed Packages Count (rpm)", "rpm -qa | wc -l", noop_follow_up),
        Unit::new("YUM/DNF Updates Available", "yum check-update 2>/dev/null | head -20 || dnf check-update 2>/dev/null | head -20", noop_follow_up),

        // Arch Linux
        Unit::new("Installed Packages (pacman)", "pacman -Q | head -20", noop_follow_up),
        Unit::new("Installed Packages Count (pacman)", "pacman -Q | wc -l", noop_follow_up),
        Unit::new("Pacman Updates Available", "pacman -Qu | head -20", noop_follow_up),

        // Snap packages (if installed)
        Unit::new("Snap Packages", "snap list 2>/dev/null | head -20", noop_follow_up),
        Unit::new("Snap Packages Count", "snap list 2>/dev/null | wc -l", noop_follow_up),

        // Flatpak packages (if installed)
        Unit::new("Flatpak Packages", "flatpak list 2>/dev/null | head -20", noop_follow_up),
        Unit::new("Flatpak Packages Count", "flatpak list 2>/dev/null | wc -l", noop_follow_up),
    ]
}
