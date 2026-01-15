use std::fmt::Write;

use crate::model::security::session::ActiveSession;

pub fn generate_active_sessions_section(
    content: &mut String,
    sessions: &Option<Vec<ActiveSession>>,
) {
    writeln!(content, "## Active Sessions").unwrap();

    let Some(sessions) = sessions else {
        writeln!(content, "No active sessions").unwrap();
        return;
    };

    if sessions.is_empty() {
        writeln!(content, "No active sessions").unwrap();
        return;
    }

    writeln!(
        content,
        "| User | TTY | From | Login At | Idle | JCPU | PCPU | What |"
    )
    .unwrap();
    writeln!(
        content,
        "|------|-----|------|----------|------|------|------|------|"
    )
    .unwrap();

    for s in sessions {
        writeln!(
            content,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            s.username,
            s.tty,
            s.from,
            s.login_at,
            s.idle,
            s.jcpu,
            s.pcpu,
            s.what
        )
        .unwrap();
    }
}

#[cfg(test)]
mod generate_active_sessions_tests {
    use super::*;

    #[test]
    fn renders_no_sessions_none() {
        let mut content = String::new();

        generate_active_sessions_section(&mut content, &None);

        assert!(content.contains("## Active Sessions"));
        assert!(content.contains("No active sessions"));
    }

    #[test]
    fn renders_no_sessions_empty() {
        let mut content = String::new();

        generate_active_sessions_section(&mut content, &Some(vec![]));

        assert!(content.contains("## Active Sessions"));
        assert!(content.contains("No active sessions"));
    }

    #[test]
    fn renders_active_sessions() {
        let mut content = String::new();

        let sessions = vec![
            ActiveSession {
                username: "alice".into(),
                tty: "pts/0".into(),
                from: "192.168.1.10".into(),
                login_at: "10:01".into(),
                idle: "00:05".into(),
                jcpu: "00:01".into(),
                pcpu: "00:00".into(),
                what: "bash".into(),
            },
            ActiveSession {
                username: "bob".into(),
                tty: "pts/1".into(),
                from: "localhost".into(),
                login_at: "09:45".into(),
                idle: ".".into(),
                jcpu: "00:10".into(),
                pcpu: "00:02".into(),
                what: "vim".into(),
            },
        ];

        generate_active_sessions_section(&mut content, &Some(sessions));

        assert!(content.contains("| User | TTY | From | Login At | Idle | JCPU | PCPU | What |"));
        assert!(content.contains("| alice | pts/0 | 192.168.1.10 | 10:01 | 00:05 | 00:01 | 00:00 | bash |"));
        assert!(content.contains("| bob | pts/1 | localhost | 09:45 | . | 00:10 | 00:02 | vim |"));
    }
}
