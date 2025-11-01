#[derive(Debug)]
struct User {
    uid: u32,
    guid: u32,
    name: String,
    home: String,
}

impl User {
    fn new(uid: u32, guid: u32, name: &str, home: &str) -> Self {
        Self {
            uid,
            guid,
            name: name.to_string(),
            home: home.to_string(),
        }
    }

    fn from_passwd_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 7 {
            return None;
        }
        Some(Self::new(
            parts[2].parse().ok()?, // uid
            parts[3].parse().ok()?, // gid
            parts[0],               // username
            parts[5],               // home
        ))
    }
}

#[derive(Debug)]
pub struct GetentPasswdFacts {
    users: Vec<User>,
}

impl GetentPasswdFacts {
    fn new() -> Self {
        Self { users: Vec::new() }
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn from_getent(output: &str) -> Self {
        let mut facts = Self::new();
        for line in output.lines() {
            if let Some(user) = User::from_passwd_line(line) {
                facts.add_user(user);
            }
        }
        facts
    }
}
