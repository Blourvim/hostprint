use crate::model::host::Host;

pub struct Unit {
    name: String,
    comand: String,
    parser: fn(stdout: &str, stderr: &str, &mut Host) -> Host,
    follow_up: fn(stdout: &str, stderr: &str, &mut Host) -> (),
}
