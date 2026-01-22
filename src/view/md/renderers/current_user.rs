use std::fmt::Write;
pub fn generate_current_user_section(
    content: &mut String,
    current_user: &Option<crate::model::security::acesss_control::SystemUser>,
) {
    writeln!(content, "## Current User\n").unwrap();

    match current_user {
        Some(user) => {
            writeln!(content, "| Field | Value |").unwrap();
            writeln!(content, "|-------|-------|").unwrap();
            writeln!(content, "| UID | {} |", user.uid.unwrap_or(0)).unwrap();
            writeln!(content, "| GID | {} |", user.gid.unwrap_or(0)).unwrap();
            writeln!(
                content,
                "| Username | {} |",
                user.name.as_deref().unwrap_or("N/A")
            )
            .unwrap();
            writeln!(
                content,
                "| Home Directory | {} |",
                user.home.as_deref().unwrap_or("N/A")
            )
            .unwrap();

            if let Some(groups) = &user.groups {

                let group_names: Vec<String> = groups
                    .iter()
                    .filter_map(|g| g.name.as_ref())
                    .cloned()
                    .collect();
                writeln!(content, "| Groups | {} |", group_names.join(", ")).unwrap();
            } else {
                writeln!(content, "| Groups | N/A |").unwrap();
            }
        }
        None => writeln!(content, "*No current user information available*\n").unwrap(),
    }
    writeln!(content, "\n---\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::security::acesss_control::{SystemGroup, SystemUser};
    use std::collections::HashSet;

    // Helper function to create a test content string
    fn test_content() -> String {
        String::new()
    }

    #[test]
    fn test_generate_current_user_section_with_user() {
        // Create a mock user with groups
        let mut groups = HashSet::new();
        groups.insert(SystemGroup {
            gid: Some(1000),
            name: Some("users".to_string()),
        });
        groups.insert(SystemGroup {
            gid: Some(1001),
            name: Some("wheel".to_string()),
        });
        groups.insert(SystemGroup {
            gid: Some(1002),
            name: Some("audio".to_string()),
        });

        let user = SystemUser {
            uid: Some(1000),
            gid: Some(1000),
            name: Some("testuser".to_string()),
            home: Some("/home/testuser".to_string()),
            groups: Some(groups),
        };

        let mut content = test_content();
        generate_current_user_section(&mut content, &Some(user));

        println!("Generated Content:\n{}", content);

        // Verify the section header
        assert!(content.contains("## Current User"));

        // Verify table structure
        assert!(content.contains("| Field | Value |"));
        assert!(content.contains("|-------|-------|"));

        // Verify specific user data
        assert!(content.contains("| UID | 1000 |"));
        assert!(content.contains("| GID | 1000 |"));
        assert!(content.contains("| Username | testuser |"));
        assert!(content.contains("| Home Directory | /home/testuser |"));

        // Verify groups (order might vary due to HashSet)
        let group_line = content
            .lines()
            .find(|line| line.contains("| Groups |"))
            .expect("Should contain groups line");
        assert!(group_line.contains("users"));
        assert!(group_line.contains("wheel"));
        assert!(group_line.contains("audio"));

        // Verify horizontal rule
        assert!(content.contains("\n---\n"));
    }

    #[test]
    fn test_generate_current_user_section_with_user_no_groups() {
        let user = SystemUser {
            uid: Some(0),
            gid: Some(0),
            name: Some("root".to_string()),
            home: Some("/root".to_string()),
            groups: None,
        };

        let mut content = test_content();
        generate_current_user_section(&mut content, &Some(user));

        println!("Generated Content (No Groups):\n{}", content);

        assert!(content.contains("## Current User"));
        assert!(content.contains("| UID | 0 |"));
        assert!(content.contains("| GID | 0 |"));
        assert!(content.contains("| Username | root |"));
        assert!(content.contains("| Home Directory | /root |"));
        assert!(content.contains("| Groups | N/A |"));
        assert!(content.contains("\n---\n"));
    }

    #[test]
    fn test_generate_current_user_section_with_user_missing_fields() {
        let user = SystemUser {
            uid: None,
            gid: None,
            name: None,
            home: None,
            groups: None,
        };

        let mut content = test_content();
        generate_current_user_section(&mut content, &Some(user));

        println!("Generated Content (Missing Fields):\n{}", content);

        assert!(content.contains("## Current User"));
        assert!(content.contains("| UID | 0 |"));
        assert!(content.contains("| GID | 0 |"));
        assert!(content.contains("| Username | N/A |"));
        assert!(content.contains("| Home Directory | N/A |"));
        assert!(content.contains("| Groups | N/A |"));
        assert!(content.contains("\n---\n"));
    }

    #[test]
    fn test_generate_current_user_section_with_none() {
        let mut content = test_content();
        generate_current_user_section(&mut content, &None);

        println!("Generated Content (None):\n{}", content);

        assert!(content.contains("## Current User"));
        assert!(content.contains("*No current user information available*"));
        assert!(content.contains("\n---\n"));

        // Should not contain table markup when None
        assert!(!content.contains("| Field | Value |"));
    }

    #[test]
    fn test_generate_current_user_section_empty_groups() {
        let user = SystemUser {
            uid: Some(1001),
            gid: Some(1001),
            name: Some("user".to_string()),
            home: Some("/home/user".to_string()),
            groups: Some(HashSet::new()), // Empty groups
        };

        let mut content = test_content();
        generate_current_user_section(&mut content, &Some(user));

        println!("Generated Content (Empty Groups):\n{}", content);

        let group_line = content
            .lines()
            .find(|line| line.contains("| Groups |"))
            .expect("Should contain groups line");
        assert!(group_line.contains("| Groups |  |"));
    }

    #[test]
    fn test_generate_current_user_section_groups_without_names() {
        let mut groups = HashSet::new();
        groups.insert(SystemGroup {
            gid: Some(999),
            name: None,
        });
        groups.insert(SystemGroup {
            gid: Some(1000),
            name: Some("validgroup".to_string()),
        });

        let user = SystemUser {
            uid: Some(1002),
            gid: Some(1002),
            name: Some("test".to_string()),
            home: Some("/home/test".to_string()),
            groups: Some(groups),
        };

        let mut content = test_content();
        generate_current_user_section(&mut content, &Some(user));

        println!("Generated Content (Groups without names):\n{}", content);

        let group_line = content
            .lines()
            .find(|line| line.contains("| Groups |"))
            .expect("Should contain groups line");
        assert!(group_line.contains("validgroup"));
        assert!(!group_line.contains("999"));
    }
}
