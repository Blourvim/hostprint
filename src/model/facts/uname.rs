#[derive(Debug, Clone, Default)]
pub struct UnameFacts {
    /// -s, --kernel-name
    pub kernel_name: Option<String>,

    /// -n, --nodename
    pub nodename: Option<String>,

    /// -r, --kernel-release
    pub kernel_release: Option<String>,

    /// -v, --kernel-version
    pub kernel_version: Option<String>,

    /// -m, --machine
    pub machine: Option<String>,

    /// -p, --processor (may be unkown by design)
    pub processor: Option<String>,

    /// -i, --hardware-platform (may be unkown by design)
    pub hardware_platform: Option<String>,

    /// -o, --operating-system
    pub operating_system: Option<String>,
}

impl UnameFacts {
    /// Create a new `UnameFacts` from a single string containing all fields separated by `\x1f`.
    pub fn new(output: String) -> Self {
        let fields: Vec<&str> = output.split('\x1f').collect();
        Self {
            kernel_name: fields.get(0).map(|s| s.to_string()),
            nodename: fields.get(1).map(|s| s.to_string()),
            kernel_release: fields.get(2).map(|s| s.to_string()),
            kernel_version: fields.get(3).map(|s| s.to_string()),
            machine: fields.get(4).map(|s| s.to_string()),
            processor: fields.get(5).map(|s| s.to_string()),
            hardware_platform: fields.get(6).map(|s| s.to_string()),
            operating_system: fields.get(7).map(|s| s.to_string()),
        }
    }
}
