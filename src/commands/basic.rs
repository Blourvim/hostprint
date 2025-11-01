use crate::{
    commands::{common::noop::noop_follow_up, unit::Unit},
    model::{
        facts::{
            os_release::{self, OsReleaseFacts},
            passwd::GetentPasswdFacts,
            uname::UnameFacts,
        },
        host::Host,
    },
};

pub fn uname_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = UnameFacts::new(stdout.into());
    println!("{:?}", facts)
}

pub fn os_release_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = OsReleaseFacts::new(stdout.into());
    println!("{:?}", facts)
}

pub fn getent_passwd_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = GetentPasswdFacts::from_getent(stdout.into());
    println!("{:?}", facts)
}
pub fn default_units() -> Vec<Unit> {
    return vec![
        Unit::new("Hostname", "hostname", noop_follow_up),
        // This ensures reliable parsing, uname -snrvmpio returns unreliable spacing
        //https://www.man7.org/linux/man-pages/man1/uname.1.html
        Unit::new(
            "System Info",
            r"uname -s && echo -n $'\x1f' &&
              uname -n && echo -n $'\x1f' &&
              uname -r && echo -n $'\x1f' &&
              uname -v && echo -n $'\x1f' &&
              uname -m && echo -n $'\x1f' &&
              uname -p && echo -n $'\x1f' &&
              uname -i && echo -n $'\x1f' &&
              uname -o",
            uname_follow_up,
        ),
        Unit::new("OS Release", "cat /etc/os-release", os_release_follow_up),
        Unit::new("Users", "getent passwd", getent_passwd_follow_up),
        Unit::new("User Info", "id", noop_follow_up),
        Unit::new("Groups", "groups", noop_follow_up),
        Unit::new("Uptime", "uptime", noop_follow_up),
        Unit::new("Logged-in Users", "w", noop_follow_up),
        Unit::new("Top Processes", "top -n 1 | head -20", noop_follow_up),
        Unit::new("Disk Usage", "df -h", noop_follow_up),
        Unit::new(
            "Largest Directories",
            "du -sh /* 2>/dev/null | sort -h | tail",
            noop_follow_up,
        ),
        Unit::new("Memory", "free -h", noop_follow_up),
        Unit::new("Network Interfaces", "ip addr", noop_follow_up),
        Unit::new("Open Ports", "ss -tuln", noop_follow_up),
        Unit::new(
            "Running Services",
            "systemctl list-units --type=service --state=running | head -30",
            noop_follow_up,
        ),
        Unit::new(
            "Top Memory Processes",
            "ps aux --sort=-%mem | head -15",
            noop_follow_up,
        ),
        Unit::new("Home Directories", "ls -lah /home", noop_follow_up),
        Unit::new("Web Roots", "ls -lah /var/www", noop_follow_up),
        Unit::new("Opt Folders", "ls -lah /opt", noop_follow_up),
        Unit::new("Logs", "ls -lah /var/log", noop_follow_up),
        Unit::new(
            "Package Manager",
            "which apt || which yum || which dnf || which pacman",
            noop_follow_up,
        ),
        Unit::new("Packages", "dpkg -l | head -20", noop_follow_up),
        Unit::new("Cron Directory", "ls -lah /etc/cron*", noop_follow_up),
        // TODO operations which require elevated permissions are out of scope for now
        // Unit::new("Cron Jobs", "sudo crontab -l", noop_follow_up),
        // Unit::new("Recent Syslog", "sudo tail -n 20 /var/log/syslog 2>/dev/null || sudo tail -n 20 /var/log/messages", noop_follow_up),

        // already have the id comand for the basic functionality
        //    Unit::new("Current User", "whoami", noop_follow_up),
    ];
}
