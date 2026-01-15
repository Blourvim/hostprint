use std::fmt::Write;

use crate::model::metrics::metrics::Metrics;

pub fn generate_metrics_section(content: &mut String, metrics: &Option<Metrics>) {
    writeln!(content, "## System Metrics").unwrap();

    let Some(metrics) = metrics else {
        writeln!(content, "No system metrics available").unwrap();
        return;
    };

    writeln!(content, "| Metric | Value |").unwrap();
    writeln!(content, "|--------|-------|").unwrap();

    writeln!(
        content,
        "| Uptime (seconds) | {} |",
        metrics.uptime_seconds.unwrap_or(0.0)
    )
    .unwrap();

    writeln!(
        content,
        "| System Time (seconds) | {} |",
        metrics.system_time_seconds.unwrap_or(0)
    )
    .unwrap();

    writeln!(
        content,
        "| Users Logged In | {} |",
        metrics.users_logged_in.unwrap_or(0)
    )
    .unwrap();

    writeln!(
        content,
        "| Load Average | {} |",
        metrics.load_average.as_deref().unwrap_or("N/A")
    )
    .unwrap();
}
#[cfg(test)]
mod generate_metrics_tests {
    use super::*;

    #[test]
    fn renders_no_metrics() {
        let mut content = String::new();

        generate_metrics_section(&mut content, &None);

        assert!(content.contains("## System Metrics"));
        assert!(content.contains("No system metrics available"));
    }

    #[test]
    fn renders_metrics_values() {
        let mut content = String::new();

        let metrics = Metrics {
            uptime_seconds: Some(1234.5),
            system_time_seconds: Some(987654),
            users_logged_in: Some(3),
            load_average: Some("0.15 0.10 0.05".into()),
        };

        generate_metrics_section(&mut content, &Some(metrics));

        assert!(content.contains("| Uptime (seconds) | 1234.5 |"));
        assert!(content.contains("| System Time (seconds) | 987654 |"));
        assert!(content.contains("| Users Logged In | 3 |"));
        assert!(content.contains("| Load Average | 0.15 0.10 0.05 |"));
    }

    #[test]
    fn renders_missing_metrics_as_defaults() {
        let mut content = String::new();

        let metrics = Metrics {
            uptime_seconds: None,
            system_time_seconds: None,
            users_logged_in: None,
            load_average: None,
        };

        generate_metrics_section(&mut content, &Some(metrics));

        assert!(content.contains("| Uptime (seconds) | 0 |"));
        assert!(content.contains("| System Time (seconds) | 0 |"));
        assert!(content.contains("| Users Logged In | 0 |"));
        assert!(content.contains("| Load Average | N/A |"));
    }
}
