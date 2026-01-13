use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupsFacts {
    pub groups: Vec<String>,
}

impl GroupsFacts {
    pub fn from_str(output: &str) -> Self {
        let groups = output
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        GroupsFacts { groups }
    }
}
