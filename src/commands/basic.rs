use crate::{commands::unit::Unit, model::host::Host};

fn noop_follow_up(stdout: &str, stderr: &str, _host: &mut Host) {
    println!("Command output:\n{}", stdout);
    if !stderr.trim().is_empty() {
        eprintln!("stderr: {}", stderr);
    }
}

pub fn default_units() -> Vec<Unit> {
    vec![
        Unit::new("Hostname", "hostname", noop_follow_up),
        Unit::new("System Info", "uname -a", noop_follow_up),
        Unit::new("OS Release", "cat /etc/os-release", noop_follow_up),
        Unit::new("Current User", "whoami", noop_follow_up),
        Unit::new("User Info", "id", noop_follow_up),
        Unit::new("Groups", "groups", noop_follow_up),
        // Unit::new("Sudo Capabilities", "sudo -l", noop_follow_up),
        Unit::new("Uptime", "uptime", noop_follow_up),
        Unit::new("Logged-in Users", "w", noop_follow_up),
        Unit::new("Top Processes", "top -n 1 | head -20", noop_follow_up),
        Unit::new("Disk Usage", "df -h", noop_follow_up),
        Unit::new("Largest Directories", "du -sh /* 2>/dev/null | sort -h | tail", noop_follow_up),
        Unit::new("Memory", "free -h", noop_follow_up),
        Unit::new("Network Interfaces", "ip addr", noop_follow_up),
        Unit::new("Open Ports", "ss -tuln", noop_follow_up),
        Unit::new("Running Services", "systemctl list-units --type=service --state=running | head -30", noop_follow_up),
        Unit::new("Top Memory Processes", "ps aux --sort=-%mem | head -15", noop_follow_up),
        Unit::new("Home Directories", "ls -lah /home", noop_follow_up),
        Unit::new("Web Roots", "ls -lah /var/www", noop_follow_up),
        Unit::new("Opt Folders", "ls -lah /opt", noop_follow_up),
        Unit::new("Logs", "ls -lah /var/log", noop_follow_up),
        // Unit::new("Recent Syslog", "sudo tail -n 20 /var/log/syslog 2>/dev/null || sudo tail -n 20 /var/log/messages", noop_follow_up),
        Unit::new("Package Manager", "which apt || which yum || which dnf || which pacman", noop_follow_up),
        Unit::new("Packages", "dpkg -l | head -20", noop_follow_up),
        // Unit::new("Cron Jobs", "sudo crontab -l", noop_follow_up),
        Unit::new("Cron Directory", "ls -lah /etc/cron*", noop_follow_up),
    ]
}
