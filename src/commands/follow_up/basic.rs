use crate::model::{
    facts::{
        df::DfFacts, du::DuFacts, id::IdFacts, os_release::OsReleaseFacts, passwd::GetentPasswdFacts, uname::UnameFacts, uptime::UptimeFacts, w::WFacts
    },
    host::Host,
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

pub fn id_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = IdFacts::from_std(stdout.into());
    println!("{:?}", facts)
}

pub fn uptime_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = UptimeFacts::from_std(stdout.into());
    println!("{:?}", facts)
}

pub fn w_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = WFacts::from_std(stdout.into());
    println!("{:?}", facts)
}

pub fn df_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = DfFacts::from_std(stdout.into());
    println!("{:?}", facts)
}


pub fn du_followup(stdout: &str, stderr: &str, host: &mut Host) -> () {
    let facts = DuFacts::from_std(stdout.into());
    println!("{:?}", facts)
}
