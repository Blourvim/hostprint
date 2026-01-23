use crate::model::network::socket::Process;
use crate::model::network::socket::Protocol;
use std::collections::HashSet;

use crate::model::facts::groups::GroupsFacts;
use crate::model::facts::whoami::WhoamiFacts;
use crate::model::network::socket::Socket;
use crate::model::{
    facts::{
        df::DfFacts, du::DuFacts, id::IdFacts, os_release::OsReleaseFacts,
        passwd::GetentPasswdFacts, ss::SsFacts, uname::UnameFacts, uptime::UptimeFacts, w::WFacts,
    },
    host::Host,
    metrics::metrics::Metrics,
    os::os::OSInfo,
    security::{
        acesss_control::{SystemGroup, SystemUser},
        session::ActiveSession,
    },
};

fn split_address(addr: &str) -> (String, Option<u16>) {
    // IPv6 addresses are like [::1]:443
    if let Some(stripped) = addr.strip_prefix('[') {
        if let Some(end) = stripped.find(']') {
            let ip = &stripped[..end];
            let rest = &stripped[end + 1..];
            let port = rest.strip_prefix(':').and_then(|p| p.parse().ok());
            return (ip.to_string(), port);
        }
    }

    // IPv4 or wildcard
    if let Some((ip, port)) = addr.rsplit_once(':') {
        return (ip.to_string(), port.parse().ok());
    }

    (addr.to_string(), None)
}
pub fn groups_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = GroupsFacts::from_str(stdout);
    let groups: Vec<SystemGroup> = facts
        .groups
        .into_iter()
        .map(|g| SystemGroup {
            name: Some(g),
            gid: None,
        })
        .collect();

    if let Some(existing_groups) = &mut host.groups {
        existing_groups.extend(groups);
    } else {
        host.groups = Some(groups);
    }
}

pub fn uname_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = UnameFacts::new(stdout.into());

    host.os = Some(OSInfo {
        name: facts.nodename.clone().or(facts.kernel_name.clone()),
        version: facts
            .kernel_release
            .clone()
            .or(facts.kernel_version.clone()),

        family: facts.operating_system.clone(),
        kernel: facts.kernel_name.clone(),

        arch: facts
            .machine
            .clone()
            .or(facts.processor.clone())
            .or(facts.hardware_platform.clone()),
    });
}
pub fn os_release_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = OsReleaseFacts::new(stdout);

    let os_info = host.os.get_or_insert(OSInfo {
        name: None,
        version: None,
        family: None,
        kernel: None,
        arch: None,
    });

    if os_info.name.is_none() {
        os_info.name = facts.pretty_name.clone().or(facts.name.clone());
    }
    if os_info.version.is_none() {
        os_info.version = facts.version_id.clone().or(facts.version.clone());
    }
    if os_info.family.is_none() {
        os_info.family = facts.id.clone();
    }
}
pub fn getent_passwd_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = GetentPasswdFacts::from_getent(stdout);

    // Map User → SystemUser
    let system_users: Vec<SystemUser> = facts
        .users
        .iter()
        .map(|u| {
            SystemUser {
                uid: Some(u.uid),
                gid: Some(u.guid),
                name: Some(u.name.clone()),
                home: Some(u.home.clone()),
                groups: None, // we'll fill later if we parse /etc/group
            }
        })
        .collect();

    // Merge with existing host.users if it exists
    if let Some(existing_users) = &mut host.users {
        existing_users.extend(system_users.clone());
    } else {
        host.users = Some(system_users.clone());
    }

    if let Some(curr_user) = host.current_user.as_mut() {
        if let Some(sys_user) = &system_users.iter().find(|p| p.name == curr_user.name) {
            curr_user.home = sys_user.home.clone();
        }
    }
}

pub fn id_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    if let Some(facts) = IdFacts::from_std(stdout) {
        if let Some(user) = host.current_user.as_mut() {
            user.name = Some(facts.name);
            user.uid = Some(facts.uid);
            user.gid = Some(facts.guid);

            let groups = user
                .groups
                .get_or_insert_with(std::collections::HashSet::new);

            for f in facts.groups {
                groups.insert(SystemGroup {
                    name: Some(f.name),
                    gid: Some(f.gid),
                });
            }
        } else {
            host.current_user = Some(SystemUser {
                name: Some(facts.name),
                uid: Some(facts.uid),
                gid: Some(facts.guid),
                home: None,
                groups: Some(
                    facts
                        .groups
                        .into_iter()
                        .map(|f| SystemGroup {
                            name: Some(f.name),
                            gid: Some(f.gid),
                        })
                        .collect::<HashSet<SystemGroup>>(),
                ),
            })
        }
    }
}

pub fn uptime_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    if let Some(facts) = UptimeFacts::from_std(stdout) {
        let metrics = Metrics {
            uptime_seconds: Some(facts.uptime_seconds),
            system_time_seconds: Some(facts.current_time_seconds),
            users_logged_in: Some(facts.users_logged_in),
            load_average: Some(facts.load_average),
        };

        host.metrics = Some(metrics)
    }
}

pub fn w_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    if let Some(facts) = WFacts::from_std(stdout) {
        let sessions: Vec<ActiveSession> = facts
            .users
            .iter()
            .map(|u| ActiveSession {
                username: u.username.clone(),
                tty: u.tty.clone(),
                from: u.from.clone(),
                login_at: u.login_at.clone(),
                idle: u.idle.clone(),
                jcpu: u.jcpu.clone(),
                pcpu: u.pcpu.clone(),
                what: u.what.clone(),
            })
            .collect();

        host.sessions = Some(sessions);
    }
}

pub fn df_follow_up(stdout: &str, _stderr: &str, _host: &mut Host) -> () {
    let _facts = DfFacts::from_std(stdout.into());
}

pub fn du_follow_up(stdout: &str, _stderr: &str, _host: &mut Host) -> () {
    let _facts = DuFacts::from_std(stdout.into());
}

pub fn ss_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = match SsFacts::from_std(stdout) {
        Some(f) => f,
        None => return,
    };

    let mut sockets = Vec::new();

    for entry in facts.entries {
        let (local_addr, local_port) = split_address(&entry.local_address);
        let (remote_addr, remote_port) = split_address(&entry.peer_address);

        let protocol = match entry.protocol.as_str() {
            "tcp" => Protocol::Tcp,
            "udp" => Protocol::Udp,
            "tcp6" => Protocol::Tcp6,
            "udp6" => Protocol::Udp6,
            _ => continue, // unknown protocol
        };

        let socket = Socket {
            protocol,

            address: local_addr,
            port: local_port.unwrap_or(0),

            remote_address: if entry.peer_address != "*" {
                Some(remote_addr)
            } else {
                None
            },
            remote_port,

            state: entry.state,

            process: Process {
                pid: None,
                uid: None,
                gid: None,
            },

            inode: 0,
            rx_queue: entry.recv_q as u32,
            tx_queue: entry.send_q as u32,
            flags: 0,

            timer_active: 0,
            timer_expire: 0,
            retransmits: 0,

            interface: None,
        };

        sockets.push(socket);
    }

    host.sockets = Some(sockets);
}
pub fn whoami_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    if let Ok(facts) = WhoamiFacts::from_std(stdout.into()) {
        if let Some(_) = &host.current_user {
            host.current_user
                .as_mut()
                .map(|user| user.name = Some(facts.username));
        } else {
            host.current_user = Some(SystemUser {
                uid: None,
                gid: None,
                name: Some(facts.username),
                home: None,
                groups: None,
            })
        }
    }
}
