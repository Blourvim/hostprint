#[derive(Debug)]
pub struct UptimeFacts {
    pub current_time_seconds: u32,
    pub uptime_seconds: f64,
    pub users_logged_in: u32,
    pub load_average: String,
}

impl UptimeFacts {
    pub fn new(
        current_time_seconds: u32,
        uptime_seconds: f64,
        users_logged_in: u32,
        load_average: String,
    ) -> Self {
        Self {
            current_time_seconds,
            uptime_seconds,
            users_logged_in,
            load_average,
        }
    }

    /// Example output:
    /// "1762061262 412067.590000 1 0.26 0.23 0.26"
    pub fn from_std(output: &str) -> Option<Self> {
        let mut parts = output.split_whitespace();

        let seconds_time = parts.next()?.parse::<u32>().ok()?;
        let uptime_seconds = parts.next()?.parse::<f64>().ok()?;
        let users_logged_in = parts.next()?.parse::<u32>().ok()?;

        let load_values: Vec<&str> = parts.collect();
        let load_average = load_values.join(" ");

        Some(Self::new(
            seconds_time,
            uptime_seconds,
            users_logged_in,
            load_average,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_facts() {
        let output = "1762061262 412067.590000 1 0.26 0.23 0.26";
        let facts = UptimeFacts::from_std(output).unwrap();

        assert_eq!(facts.current_time_seconds, 1762061262);
        assert!((facts.uptime_seconds - 412067.59).abs() < f64::EPSILON);
        assert_eq!(facts.users_logged_in, 1);
        assert_eq!(facts.load_average, "0.26 0.23 0.26");
    }
}
