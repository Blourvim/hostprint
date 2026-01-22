use std::fmt::Write;

use crate::{
    model::host::Host,
    view::md::renderers::{
        current_user::generate_current_user_section, groups::generate_groups_section,
        metrics::generate_metrics_section, os::generate_os_section,
        sesssions::generate_active_sessions_section, sockets::generate_sockets_section,
    },
};

pub struct Md {
    content: String,
}

impl Md {
    pub fn new(host: &Host) -> Self {
        let mut content = String::new();
        Self::generate_markdown(&mut content, &host);
        Self { content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn generate_markdown(content: &mut String, host: &Host) {
        writeln!(content, "# Host System Information\n").unwrap();
        writeln!(content, "*Generated from system inventory data*\n").unwrap();
        generate_current_user_section(content, &host.current_user);
        generate_groups_section(content, &host.groups);
        generate_metrics_section(content, &host.metrics);
        generate_os_section(content, &host.os);
        generate_active_sessions_section(content, &host.sessions);
        generate_sockets_section(content, &host.sockets);
    }
}
