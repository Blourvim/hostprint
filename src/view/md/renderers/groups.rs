fn generate_groups_section(
    content: &mut String,
    _groups: &Option<Vec<crate::model::security::acesss_control::SystemGroup>>,
) {
    content.push_str("System Groups\n");
}
#[cfg(test)]
mod generate_groups_tests{
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
}
