use crate::commands::{
    common::noop::noop_follow_up,
    follow_up::{
        basic::{
            getent_passwd_follow_up, groups_follow_up, id_follow_up, os_release_follow_up,
            ss_follow_up, uname_follow_up, uptime_follow_up, w_follow_up, whoami_follow_up,
        },
        network::network_interfaces_follow_up,
    },
    unit::Unit,
};

pub fn default_units() -> Vec<Unit> {
    return vec![
        Unit::new("Hostname", "hostname", noop_follow_up),
        Unit::new("Current User", "whoami", whoami_follow_up),
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
        Unit::new("Uptime", "uptime --raw", uptime_follow_up),
        // TODO w for containers out of scope for now
        Unit::new("Logged-in Users", "w -h ", w_follow_up),
        Unit::new("Open Ports", "ss -HtulnpO", ss_follow_up),
        Unit::new(
            "Network Interfaces",
            "ip addr",
            network_interfaces_follow_up,
        ),
        Unit::new("Groups", "groups", groups_follow_up),
    ];
}
