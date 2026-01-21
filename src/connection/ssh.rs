use std::io::BufReader;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use crate::connection::common::ShellTransport;

#[derive(Debug, Clone)]
pub enum SSHAuth {
    PrivateKey {
        path: PathBuf,
        passphrase: Option<String>,
    },
    Password(String),
}

#[derive(Debug, Clone)]
pub struct SSHClient {
    destination: String,
    port: u16,
    username: String,
    auth: SSHAuth,
    command: String,
    timeout_secs: Option<u64>,
    verbose: bool,
}

pub struct SSHConnection {
    child: Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}

impl SSHClient {
    pub fn new(destination: String) -> SSHClient {
        Self {
            destination,
            command: "uname -a".into(),
            port: 22,
            username: "root".into(),
            verbose: false,
            timeout_secs: Some(10),
            auth: SSHAuth::PrivateKey {
                path: PathBuf::from("~/.ssh/id_rsa"),
                passphrase: None,
            },
        }
    }
}

impl SSHClient {
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

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }
}
impl SSHClient {
    pub fn open_shell(&self) -> std::io::Result<SSHConnection> {
        let mut cmd = Command::new("ssh");

        if self.verbose {
            cmd.arg("-v");
        }

        cmd.arg("-p").arg(self.port.to_string());
        cmd.arg(format!("{}@{}", self.username, self.destination));

        if let Some(timeout) = self.timeout_secs {
            cmd.arg("-o").arg(format!("ConnectTimeout={}", timeout));
        }

        match &self.auth {
            SSHAuth::PrivateKey {
                path,
                passphrase: None,
            } => {
                cmd.arg("-i").arg(path);
            }
            SSHAuth::Password(_) => {
                panic!("Password auth not implemented");
            }

            SSHAuth::PrivateKey {
                path: _,
                passphrase: Some(_),
            } => {
                panic!("Passphrase is not yet supported");
            }
        }

        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().expect("stdin not captured");
        let stdout = BufReader::new(child.stdout.take().expect("stdout not captured"));

        Ok(SSHConnection {
            child,
            stdin,
            stdout,
        })
    }
}

impl SSHConnection {
    pub fn child(&self) -> &Child {
        &self.child
    }
}

impl ShellTransport for SSHConnection {
    fn stdin(&mut self) -> &mut dyn std::io::Write {
        &mut self.stdin
    }

    fn stdout(&mut self) -> &mut dyn std::io::BufRead {
        &mut self.stdout
    }
}
