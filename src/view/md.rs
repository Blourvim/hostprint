use std::fmt::Write;
pub mod md;
pub mod renderers;

use crate::model::host::Host;

pub struct Md {
    content: String,
}

impl Md {
    pub fn new(host: Host) -> Self {
        let mut content = String::new();
        Self::generate_markdown(&mut content, &host);
        Self { content }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn generate_markdown(content: &mut String, _host: &Host) {
        writeln!(content, "# Host System Information\n").unwrap();
        writeln!(content, "*Generated from system inventory data*\n").unwrap();

        todo!();

        // Current User Section
    }
}
