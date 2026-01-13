use crate::commands::{
    common::noop::noop_follow_up,
    follow_up::{
        basic::{
            df_follow_up, du_follow_up, getent_passwd_follow_up, groups_follow_up, id_follow_up,
            os_release_follow_up, ss_follow_up, uname_follow_up, uptime_follow_up, w_follow_up,
        },
        network::network_interfaces_follow_up,
    },
    unit::Unit,
};

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
        Unit::new("User Info", "id", id_follow_up),
        // TODO uptime for containers out of scope for now
        Unit::new("Uptime", "uptime", uptime_follow_up),
        // TODO w for containers out of scope for now
        Unit::new("Logged-in Users", "w -h ", w_follow_up),
        Unit::new("Disk Usage", "df", df_follow_up),
        Unit::new(
            "Largest Directories",
            "du -sh --time /* 2>/dev/null",
            du_follow_up,
        ),
        Unit::new("Open Ports", "ss -HtulnpO", ss_follow_up),
        Unit::new("Network Interfaces", "ip addr", network_interfaces_follow_up),
        //  OUT OF SCOPE
        //  -----------------------------------
        // Unit::new(
        //     "Running Services",
        //     "systemctl list-units --type=service --state=running",
        //     noop_follow_up,
        // ),
        // Unit::new(
        //     "Top Memory Processes",
        //     "ps aux --sort=-%mem | head -15",
        //     noop_follow_up,
        // ),
        //Unit::new("Memory", "free -h", noop_follow_up),
        //Unit::new("Top Processes", "top -n 1 | head -20", noop_follow_up),
        //Unit::new("Home Directories", "ls -lah /home", noop_follow_up),
        //Unit::new("Web Roots", "ls -lah /var/www", noop_follow_up),
        //Unit::new("Opt Folders", "ls -lah /opt", noop_follow_up),
        //Unit::new("Logs", "ls -lah /var/log", noop_follow_up),
        //Unit::new(
        //    "Package Manager",
        //    "which apt || which yum || which dnf || which pacman",
        //    noop_follow_up,
        //),
        //Unit::new("Packages", "dpkg -l | head -20", noop_follow_up),
        //Unit::new("Cron Directory", "ls -lah /etc/cron*", noop_follow_up),
        // TODO operations which require elevated permissions are out of scope for now
        // Unit::new("Cron Jobs", "sudo crontab -l", noop_follow_up),
        // Unit::new("Recent Syslog", "sudo tail -n 20 /var/log/syslog 2>/dev/null || sudo tail -n 20 /var/log/messages", noop_follow_up),

        // already implemented another comand for it unnecesary for now
        Unit::new("Current User", "whoami", noop_follow_up),
        Unit::new("Groups", "groups", groups_follow_up),
    ];
}
