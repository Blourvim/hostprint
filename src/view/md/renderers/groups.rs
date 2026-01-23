pub fn generate_groups_section(
    content: &mut String,
    groups: &Option<Vec<crate::model::security::acesss_control::SystemGroup>>,
) {
    content.push_str("## System Groups\n");

    if let Some(groups) = groups {
        if groups.is_empty() {
            content.push_str("No group information available");
        } else {
            content.push_str("| GID | Group Name |\n");
            content.push_str("| --- | ---------- |\n");

            for group in groups {
                let gid = group
                    .gid
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                let name = group.name.as_deref().unwrap_or("N/A");

                content.push_str(&format!("| {} | {} |\n", gid, name));
            }
        }
    } else {
        content.push_str("No group information available");
    }
}
#[cfg(test)]
mod generate_groups_tests {
    use super::*;

    use crate::model::security::acesss_control::SystemGroup;

    #[test]
    fn generates_groups_section_with_groups() {
        let mut content = String::new();

        let groups = Some(vec![
            SystemGroup {
                gid: Some(100),
                name: Some("admins".to_string()),
            },
            SystemGroup {
                gid: None,
                name: None,
            },
        ]);

        generate_groups_section(&mut content, &groups);

        assert!(content.contains("## System Groups"));
        assert!(content.contains("| GID | Group Name |"));
        assert!(content.contains("| 100 | admins |"));
        assert!(content.contains("| 0 | N/A |"));
    }

    #[test]
    fn generates_groups_section_without_groups() {
        let mut content = String::new();

        generate_groups_section(&mut content, &None);

        assert!(content.contains("## System Groups"));
        assert!(content.contains("No group information available"));
    }

    #[test]
    fn generates_groups_section_with_empty_groups() {
        let mut content = String::new();

        generate_groups_section(&mut content, &Some(vec![]));

        assert!(content.contains("## System Groups"));
        assert!(content.contains("No group information available"));
    }
}
