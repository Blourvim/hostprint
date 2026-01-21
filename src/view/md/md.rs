use std::fmt::Write;

use crate::{
    model::host::Host,
    view::md::renderers::{self, current_user::generate_current_user_section, groups::generate_groups_section, metrics::generate_metrics_section},
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

        println!("{:?}", content)
    }
}
