#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct ActiveSession {
    pub username: String,
    pub tty: String,
    pub from: String,
    pub login_at: String,
    pub idle: String,
    pub jcpu: String,
    pub pcpu: String,
    pub what: String,
}
