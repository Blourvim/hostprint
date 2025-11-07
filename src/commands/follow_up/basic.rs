use crate::model::{
    facts::{
        df::DfFacts, du::DuFacts, id::IdFacts, os_release::OsReleaseFacts,
        passwd::GetentPasswdFacts, ss::SsFacts, uname::UnameFacts, uptime::UptimeFacts, w::WFacts,
    },
    host::Host,
    os::os::OSInfo,
};

pub fn uname_follow_up(stdout: &str, stderr: &str, host: &mut Host) {
    let facts = UnameFacts::new(stdout.into());

    host.os = Some(OSInfo {
        // hostname or kernel name
        name: facts.nodename.clone().or(facts.kernel_name.clone()),

        // kernel release or version
        version: facts
            .kernel_release
            .clone()
            .or(facts.kernel_version.clone()),

        // OS family (GNU/Linux, Darwin, etc.)
        family: facts.operating_system.clone(),

        // kernel name
        kernel: facts.kernel_name.clone(),

        // architecture or machine info
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
