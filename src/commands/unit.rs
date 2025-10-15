use crate::model::host::Host;

pub struct Unit {
    pub name: String,
    pub comand: String,
    pub follow_up: fn(stdout: &str, stderr: &str, &mut Host),
}

impl Unit {
    pub fn new(
        name: &str,
        comand: &str,
        follow_up: fn(stdout: &str, stderr: &str, &mut Host),
    ) -> Self {
        Self {
            name: name.to_string(),
            comand: comand.to_string(),
            follow_up,
        }
    }
}
