#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Metrics {
    pub uptime_seconds: Option<f64>,
    pub current_time_seconds: Option<u32>,
    pub users_logged_in: Option<u32>,
    pub load_average: Option<String>,
}
