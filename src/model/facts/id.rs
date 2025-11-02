#[derive(Debug)]
pub struct Group {
    pub gid: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct IdFacts {
    pub uid: u32,
    pub guid: u32,
    pub name: String,
    pub groups: Vec<Group>,
}

impl Group {
    /// Parse a string like "1000(devel),27(sudo),100(users),1003(hostshare_group)"
    pub fn from_groups_string(s: &str) -> Vec<Self> {
        s.split(',')
            .filter_map(|grp| {
                let start = grp.find('(')?;
                let end = grp.find(')')?;
                let gid = grp[..start].parse::<u32>().ok()?;
                let name = grp[start + 1..end].to_string();
                Some(Self { gid, name })
            })
            .collect()
    }
}

impl IdFacts {
    pub fn new(uid: u32, guid: u32, name: &str, groups: Vec<Group>) -> Self {
        Self {
            uid,
            guid,
            name: name.to_string(),
            groups,
        }
    }

    pub fn from_std(output: &str) -> Option<Self> {
        let mut uid = None;
        let mut guid = None;
        let mut name = None;
        let mut groups = Vec::new();

        for part in output.split_whitespace() {
            if let Some(stripped) = part.strip_prefix("uid=") {
                let paren = stripped.find('(')?;
                uid = stripped[..paren].parse().ok();
                name = Some(stripped[paren + 1..stripped.len() - 1].to_string());
            } else if let Some(stripped) = part.strip_prefix("gid=") {
                let paren = stripped.find('(')?;
                guid = stripped[..paren].parse().ok();
            } else if let Some(stripped) = part.strip_prefix("groups=") {
                groups = Group::from_groups_string(stripped);
            }
        }

        Some(Self::new(uid?, guid?, &name?, groups))
    }
}
