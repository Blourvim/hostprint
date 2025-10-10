use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SSHOptions {
    pub destination: String,
    pub port: u16,
    pub username: String,
    pub auth: SSHAuth,
    pub command: String,
    pub timeout_secs: Option<u64>,
    pub strict_host_key_checking: bool,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum SSHAuth {
    Password(String),
    PrivateKey {
        path: PathBuf,
        passphrase: Option<String>,
    },
}

impl SSHOptions {
    pub fn new(destination: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            destination: destination.into(),
            command: command.into(),
            port: 22,
            username: "root".into(),
            auth: SSHAuth::PrivateKey {
                path: PathBuf::from("~/.ssh/id_rsa"),
                passphrase: None,
            },
            timeout_secs: Some(30),
            strict_host_key_checking: true,
            verbose: false,
        }
    }

    pub fn with_port(mut self, port: impl Into<u16>) -> Self {
        self.port = port.into();
        self
    }

    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    pub fn with_private_key(mut self, path: impl Into<PathBuf>) -> Self {
        self.private_key = Some(path.into());
        self
    }

    pub fn disable_strict_host_checking(mut self) -> Self {
        self.strict_host_key_checking = false;
        self
    }
}
