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
