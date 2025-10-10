use std::{io::Stderr, path::PathBuf, process::Command};

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
        self.auth = SSHAuth::PrivateKey {
            path: path.into(),
            passphrase: None,
        };
        self
    }

    pub fn disable_strict_host_checking(mut self) -> Self {
        self.strict_host_key_checking = false;
        self
    }

    pub fn exec(&self) -> Result<String, String> {
        let output = Command::new("ls").output().map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }
}
