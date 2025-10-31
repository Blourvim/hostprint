#[derive(Debug, Clone, Default)]
/// https://www.linux.org/docs/man5/os-release.html
pub struct OsReleaseFacts {
    pub name: Option<String>,               // NAME
    pub version: Option<String>,            // VERSION
    pub id: Option<String>,                 // ID
    pub version_id: Option<String>,         // VERSION_ID
    pub pretty_name: Option<String>,        // PRETTY_NAME
    pub ansi_color: Option<String>,         // ANSI_COLOR
    pub cpe_name: Option<String>,           // CPE_NAME
    pub home_url: Option<String>,           // HOME_URL
    pub support_url: Option<String>,        // SUPPORT_URL
    pub bug_report_url: Option<String>,     // BUG_REPORT_URL
    pub privacy_policy_url: Option<String>, // PRIVACY_POLICY_URL
    pub build_id: Option<String>,           // BUILD_ID
    pub variant: Option<String>,            // VARIANT
    pub variant_id: Option<String>,         // VARIANT_ID
    // Optional: store unrecognized fields as a fallback
    pub other: std::collections::HashMap<String, String>,
}

/// https://www.linux.org/docs/man5/os-release.html
impl OsReleaseFacts {
    pub fn new(output: &str) -> Self {
        let mut release = Self {
            other: std::collections::HashMap::new(),
            ..Default::default()
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let value = value.trim_matches('"').to_string();

                match key {
                    "NAME" => release.name = Some(value),
                    "VERSION" => release.version = Some(value),
                    "ID" => release.id = Some(value),
                    "VERSION_ID" => release.version_id = Some(value),
                    "PRETTY_NAME" => release.pretty_name = Some(value),
                    "ANSI_COLOR" => release.ansi_color = Some(value),
                    "CPE_NAME" => release.cpe_name = Some(value),
                    "HOME_URL" => release.home_url = Some(value),
                    "SUPPORT_URL" => release.support_url = Some(value),
                    "BUG_REPORT_URL" => release.bug_report_url = Some(value),
                    "PRIVACY_POLICY_URL" => release.privacy_policy_url = Some(value),
                    "BUILD_ID" => release.build_id = Some(value),
                    "VARIANT" => release.variant = Some(value),
                    "VARIANT_ID" => release.variant_id = Some(value),
                    _ => {
                        release.other.insert(key.to_string(), value);
                    } // unknown fields
                }
            }
        }

        release
    }
}
