#[derive(Debug)]
pub struct WUser {
    pub username: String,
    pub tty: String,
    pub from: String,
    pub login_at: String,
    pub idle: String,
    pub jcpu: String,
    pub pcpu: String,
    pub what: String,
}

#[derive(Debug)]
pub struct WFacts {
    pub users: Vec<WUser>,
}

impl WUser {
    /// Parse one line of `w -h` output.
    pub fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.split_whitespace();
        let username = parts.next()?.to_string();
        let tty = parts.next()?.to_string();
        let from = parts.next()?.to_string();
        let login_at = parts.next()?.to_string();
        let idle = parts.next()?.to_string();
        let jcpu = parts.next()?.to_string();
        let pcpu = parts.next()?.to_string();
        let what = parts.collect::<Vec<_>>().join(" ");
        Some(Self {
            username,
            tty,
            from,
            login_at,
            idle,
            jcpu,
            pcpu,
            what,
        })
    }
}

impl WFacts {
    pub fn from_std(output: &str) -> Option<Self> {
        let users = output
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(WUser::from_line)
            .collect::<Vec<_>>();

        Some(Self { users })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wfacts_parsing() {
        let sample = r#"
devel    ttyS0    -                Sat17    3.00s  6.08s   ?    w -h
"#;

        let facts = WFacts::from_std(sample).unwrap();

        assert_eq!(facts.users.len(), 1);
        assert_eq!(facts.users[0].username, "devel");
        assert_eq!(facts.users[0].what, "w -h");
    }
}
