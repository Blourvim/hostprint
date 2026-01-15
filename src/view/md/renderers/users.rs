use std::fmt::Write;
fn generate_users_section(
    content: &mut String,
    users: &Option<Vec<crate::model::security::acesss_control::SystemUser>>,
) {
    writeln!(content, "## System Users\n").unwrap();

    match users {
        Some(users_list) => {
            writeln!(
                content,
                "| UID | GID | Username | Home Directory | Groups |"
            )
            .unwrap();
            writeln!(
                content,
                "|-----|-----|----------|----------------|--------|"
            )
            .unwrap();
            for user in users_list {
                let groups = user
                    .groups
                    .as_ref()
                    .map(|g| {
                        g.iter()
                            .filter_map(|grp| grp.name.as_ref())
                            .cloned()
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "N/A".to_string());

                writeln!(
                    content,
                    "| {} | {} | {} | {} | {} |",
                    user.uid.unwrap_or(0),
                    user.gid.unwrap_or(0),
                    user.name.as_deref().unwrap_or("N/A"),
                    user.home.as_deref().unwrap_or("N/A"),
                    groups
                )
                .unwrap();
            }
        }
        None => writeln!(content, "*No user information available*\n").unwrap(),
    }
    writeln!(content, "\n---\n").unwrap();
}
