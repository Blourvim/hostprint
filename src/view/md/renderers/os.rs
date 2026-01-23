use std::fmt::Write;

use crate::model::os::os::OSInfo;

pub fn generate_os_section(content: &mut String, os: &Option<OSInfo>) {
    writeln!(content, "## Operating System").unwrap();

    let Some(os) = os else {
        writeln!(content, "No operating system information available").unwrap();
        return;
    };

    writeln!(content, "| Property | Value |").unwrap();
    writeln!(content, "|----------|-------|").unwrap();

    writeln!(
        content,
        "| Architecture | {} |",
        os.arch.as_deref().unwrap_or("N/A")
    )
    .unwrap();
    writeln!(
        content,
        "| Family | {} |",
        os.family.as_deref().unwrap_or("N/A")
    )
    .unwrap();

    writeln!(
        content,
        "| Kernel | {} |",
        os.kernel.as_deref().unwrap_or("N/A")
    )
    .unwrap();
    writeln!(
        content,
        "| Version | {} |",
        os.version.as_deref().unwrap_or("N/A")
    )
    .unwrap();

    writeln!(
        content,
        "| Name | {} |",
        os.name.as_deref().unwrap_or("N/A")
    )
    .unwrap();
}

#[cfg(test)]
mod generate_os_tests {
    use super::*;

    #[test]
    fn renders_no_os_info() {
        let mut content = String::new();

        generate_os_section(&mut content, &None);

        assert!(content.contains("## Operating System"));
        assert!(content.contains("No operating system information available"));
    }

    #[test]
    fn renders_basic_os_info() {
        let mut content = String::new();

        let os = OSInfo {
            name: Some("Linux".into()),
            version: Some("6.6".into()),
            family: Some("Unix".into()),
            kernel: Some("Linux".into()),
            arch: Some("x86_64".into()),
        };

        generate_os_section(&mut content, &Some(os));

        assert!(content.contains("| Name | Linux |"));
        assert!(content.contains("| Version | 6.6 |"));
        assert!(content.contains("| Family | Unix |"));
        assert!(content.contains("| Kernel | Linux |"));
        assert!(content.contains("| Architecture | x86_64 |"));
    }

    #[test]
    fn renders_missing_fields_as_na() {
        let mut content = String::new();

        let os = OSInfo {
            name: None,
            version: None,
            family: None,
            kernel: None,
            arch: None,
        };

        generate_os_section(&mut content, &Some(os));

        assert!(content.contains("| Name | N/A |"));
        assert!(content.contains("| Version | N/A |"));
        assert!(content.contains("| Architecture | N/A |"));
    }
}
