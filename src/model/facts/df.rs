#[derive(Debug, Clone, Default)]
pub struct DfFacts {
    pub filesystems: Vec<FileSystem>,
}

impl DfFacts {
    pub fn from_std(output: String) -> Self {
        let mut filesystems = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        // Skip header line
        for line in lines.iter().skip(1) {
            if let Some(fs) = FileSystem::from_line(line) {
                filesystems.push(fs);
            }
        }

        Self { filesystems }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileSystem {
    pub filesystem: Option<String>,
    /// Total size of the file system in 1K blocks 
    pub size: Option<String>,
    pub used: Option<String>,
    pub available: Option<String>,
    pub use_percent: Option<String>,
    pub mounted_on: Option<String>,
}

impl FileSystem {
    pub fn from_line(line: &str) -> Option<Self> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.is_empty() {
            return None;
        }

        Some(Self {
            filesystem: fields.get(0).map(|s| s.to_string()),
            size: fields.get(1).map(|s| s.to_string()),
            used: fields.get(2).map(|s| s.to_string()),
            available: fields.get(3).map(|s| s.to_string()),
            use_percent: fields.get(4).map(|s| s.to_string()),
            mounted_on: fields.get(5).map(|s| s.to_string()),
        })
    }
}
