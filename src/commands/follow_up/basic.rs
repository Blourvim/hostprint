use crate::model::{
    facts::{
        df::DfFacts, du::DuFacts, id::IdFacts, os_release::OsReleaseFacts,
        passwd::GetentPasswdFacts, ss::SsFacts, uname::UnameFacts, uptime::UptimeFacts, w::WFacts,
    },
    host::Host,
    metrics::metrics::Metrics,
    os::os::OSInfo,
    security::{acesss_control::SystemUser, session::ActiveSession},
};

pub fn uname_follow_up(stdout: &str, stderr: &str, host: &mut Host) {
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
pub fn os_release_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = OsReleaseFacts::new(stdout.into());
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
        existing_users.extend(system_users);
    } else {
        host.users = Some(system_users);
    }

    // Optionally, we could guess "current user" by UID == 0 or matching `whoami`
    if host.current_user.is_none() {
        // crude heuristic: root or first user
        let maybe_current = host
            .users
            .as_ref()
            .and_then(|u| u.iter().find(|usr| usr.uid == Some(0)))
            .cloned()
            .or_else(|| host.users.as_ref().and_then(|u| u.first().cloned()));

        host.current_user = maybe_current;
    }
}

pub fn id_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = IdFacts::from_std(stdout.into());
}

pub fn uptime_followup(stdout: &str, _stderr: &str, host: &mut Host) {
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

pub fn w_followup(stdout: &str, _stderr: &str, host: &mut Host) {
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

pub fn df_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = DfFacts::from_std(stdout.into());
}

pub fn du_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = DuFacts::from_std(stdout.into());
}

pub fn ss_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = SsFacts::from_std(stdout.into());
}
