use crate::model::{
    facts::{
        df::DfFacts, du::DuFacts, id::IdFacts, os_release::OsReleaseFacts,
        passwd::GetentPasswdFacts, ss::SsFacts, uname::UnameFacts, uptime::UptimeFacts, w::WFacts,
    },
    host::{Host, Os},
};

pub fn uname_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = UnameFacts::new(stdout.into());

    host.os =
}

pub fn os_release_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = OsReleaseFacts::new(stdout.into());
}

pub fn getent_passwd_follow_up(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = GetentPasswdFacts::from_getent(stdout.into());
}

pub fn id_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = IdFacts::from_std(stdout.into());
}

pub fn uptime_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = UptimeFacts::from_std(stdout.into());
}

pub fn w_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = WFacts::from_std(stdout.into());
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
