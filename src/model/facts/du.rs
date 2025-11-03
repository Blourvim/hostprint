use std::str;

#[derive(Debug, Clone)]
pub struct DuFacts {
    pub entries: Vec<DuEntry>,
}

#[derive(Debug, Clone)]
pub struct DuEntry {
    pub path: String,
    pub size: String,
    pub last_modified: Option<String>,
}

impl DuFacts {
    pub fn from_std(output: &str) -> DuFacts {
        let entries: Vec<DuEntry> = output
            .lines()
            .filter_map(|line| DuEntry::from_line(line).ok())
            .collect();

        DuFacts { entries }
    }

    pub fn from_entries(entries: Vec<DuEntry>) -> DuFacts {
        DuFacts { entries }
    }
}

impl DuEntry {
    pub fn from_line(line: &str) -> Result<Self, String> {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            return Err("Empty line".to_string());
        }

        let mut parts = trimmed_line.split_whitespace();

        // The first part should be the size
        let size = parts
            .next()
            .ok_or_else(|| "Missing size field".to_string())?
            .to_string();

        let mut potential_date = None;
        let mut potential_time = None;
        let mut path_parts = Vec::new();

        // date parsing
        while let Some(part) = parts.next() {
            if part.contains('-') && potential_date.is_none() {
                potential_date = Some(part);
            } else if part.contains(':') && potential_time.is_none() {
                potential_time = Some(part);
            } else {
                path_parts.push(part);
            }
        }

        let path = path_parts.join(" ");
        if path.is_empty() {
            return Err("Missing path field".to_string());
        }

        let last_modified = match (potential_date, potential_time) {
            (Some(date), Some(time)) => Some(format!("{} {}", date, time)),
            _ => None,
        };

        Ok(DuEntry {
            path,
            size,
            last_modified,
        })
    }

    pub fn new(path: String, size: String, last_modified: Option<String>) -> Self {
        DuEntry {
            path,
            size,
            last_modified,
        }
    }
}
