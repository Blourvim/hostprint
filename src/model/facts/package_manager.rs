use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManagerFacts {
    pub found_path: Option<String>,
}

impl PackageManagerFacts {
    pub fn from_str(output: &str) -> Self {
        let output = output.trim();
        if output.is_empty() {
            return PackageManagerFacts { found_path: None };
        }
        PackageManagerFacts {
            found_path: Some(output.to_string()),
        }
    }
}
