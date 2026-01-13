use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFact {
    pub name: String,
    pub version: String,
    pub architecture: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagesFacts {
    pub packages: Vec<PackageFact>,
}

impl PackagesFacts {
    pub fn from_dpkg(output: &str) -> Self {
        let mut packages = Vec::new();
        for line in output.lines() {
            if !line.starts_with("ii") {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                // ii  name  version  arch  description...
                let name = parts[1].to_string();
                let version = parts[2].to_string();
                let architecture = Some(parts[3].to_string());
                let description = Some(parts[4..].join(" "));
                
                packages.push(PackageFact {
                    name,
                    version,
                    architecture,
                    description,
                    status: Some("installed".to_string()),
                });
            }
        }
        PackagesFacts { packages }
    }
}
