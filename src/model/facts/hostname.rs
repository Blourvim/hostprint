#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostnameFacts {
    pub hostname: String,
}

impl HostnameFacts {
    pub fn from_std(output: &str) -> Result<Self, String> {
        let mut seen = None;

        for (idx, raw_line) in output.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() {
                continue;
            }

            let hostname =
                Self::parse_hostname(line).map_err(|e| format!("line {}: {}", idx + 1, e))?;

            if seen.is_some() {
                return Err("hostname produced multiple values".to_string());
            }

            seen = Some(hostname);
        }

        match seen {
            Some(hostname) => Ok(HostnameFacts { hostname }),
            None => Err("hostname produced no output".to_string()),
        }
    }

    fn parse_hostname(line: &str) -> Result<String, String> {
        // NOTE: intentionally weak validation for now
        if line.is_empty() {
            return Err("hostname is empty".to_string());
        }

        Ok(line.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_hostname() {
        let input = "myhost\n";
        let facts = HostnameFacts::from_std(input).unwrap();
        assert_eq!(facts.hostname, "myhost");
    }

    #[test]
    fn trims_whitespace() {
        let input = "   myhost   \n";
        let facts = HostnameFacts::from_std(input).unwrap();
        assert_eq!(facts.hostname, "myhost");
    }

    #[test]
    fn rejects_multiple_lines() {
        let input = "host1\nhost2\n";
        let err = HostnameFacts::from_std(input).unwrap_err();
        assert!(err.contains("multiple"));
    }

    // --- failing tests below ---

    #[test]
    fn rejects_empty_hostname() {
        let input = "\n   \n";
        let err = HostnameFacts::from_std(input).unwrap_err();
        assert!(err.contains("no output"));
    }
}
