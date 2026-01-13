#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhoamiFacts {
    pub username: String,
}

impl WhoamiFacts {
    pub fn from_std(output: &str) -> Result<Self, String> {
        let mut seen = None;

        for (idx, raw_line) in output.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            let username =
                Self::parse_username(line).map_err(|e| format!("line {}: {}", idx + 1, e))?;

            if seen.is_some() {
                return Err("whoami produced multiple usernames".to_string());
            }

            seen = Some(username);
        }

        match seen {
            Some(username) => Ok(WhoamiFacts { username }),
            None => Err("whoami produced no output".to_string()),
        }
    }

    fn parse_username(line: &str) -> Result<String, String> {
        if line.chars().any(|c| c.is_control()) {
            return Err(format!("username contains control characters: {:?}", line));
        }

        if line.split_whitespace().count() != 1 {
            return Err(format!("username contains whitespace: {:?}", line));
        }

        Ok(line.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_username() {
        let input = "root\n";
        let facts = WhoamiFacts::from_std(input).unwrap();
        assert_eq!(facts.username, "root");
    }

    #[test]
    fn trims_whitespace_around_username() {
        let input = "   alice   \n";
        let facts = WhoamiFacts::from_std(input).unwrap();
        assert_eq!(facts.username, "alice");
    }

    #[test]
    fn ignores_empty_lines() {
        let input = "\n\nbob\n\n";
        let facts = WhoamiFacts::from_std(input).unwrap();
        assert_eq!(facts.username, "bob");
    }

    #[test]
    fn rejects_whitespace_inside_username() {
        let input = "not valid\n";
        let err = WhoamiFacts::from_std(input).unwrap_err();
        assert!(err.contains("whitespace"));
    }

    #[test]
    fn rejects_multiple_usernames() {
        let input = "alice\nbob\n";
        let err = WhoamiFacts::from_std(input).unwrap_err();
        assert!(err.contains("multiple"));
    }

    #[test]
    fn rejects_empty_output() {
        let input = "\n   \n";
        let err = WhoamiFacts::from_std(input).unwrap_err();
        assert!(err.contains("no output"));
    }

    #[test]
    fn rejects_control_characters() {
        let input = "ali\u{0000}ce\n";
        let err = WhoamiFacts::from_std(input).unwrap_err();
        assert!(err.contains("control"));
    }
}

