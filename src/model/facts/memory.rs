use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFacts {
    pub total: Option<String>,
    pub used: Option<String>,
    pub free: Option<String>,
    pub shared: Option<String>,
    pub buff_cache: Option<String>,
    pub available: Option<String>,
    pub swap_total: Option<String>,
    pub swap_used: Option<String>,
    pub swap_free: Option<String>,
}

impl MemoryFacts {
    pub fn from_str(output: &str) -> Self {
        let mut lines = output.lines();
        // Skip header
        lines.next();

        let mut facts = MemoryFacts {
            total: None,
            used: None,
            free: None,
            shared: None,
            buff_cache: None,
            available: None,
            swap_total: None,
            swap_used: None,
            swap_free: None,
        };

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            if parts[0].starts_with("Mem:") {
                if parts.len() >= 7 {
                    facts.total = Some(parts[1].to_string());
                    facts.used = Some(parts[2].to_string());
                    facts.free = Some(parts[3].to_string());
                    facts.shared = Some(parts[4].to_string());
                    facts.buff_cache = Some(parts[5].to_string());
                    facts.available = Some(parts[6].to_string());
                }
            } else if parts[0].starts_with("Swap:") {
                if parts.len() >= 4 {
                    facts.swap_total = Some(parts[1].to_string());
                    facts.swap_used = Some(parts[2].to_string());
                    facts.swap_free = Some(parts[3].to_string());
                }
            }
        }

        facts
    }
}
