use std::{path::PathBuf, process::Command};

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Stdio};

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

#[derive(Debug, Clone)]
pub enum SSHAuth {
    Password(String),
    PrivateKey {
        path: PathBuf,
        passphrase: Option<String>,
    },
}
impl SSHClient {
    pub fn new(destination: impl Into<String>) -> Self {
        Self {
            destination: destination.into(),
            command: "uname -a".into(),
            port: 22,
            username: "root".into(),
            auth: SSHAuth::PrivateKey {
                path: PathBuf::from("~/.ssh/id_rsa"),
                passphrase: None,
            },
            timeout_secs: Some(30),
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

    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = command.into();
        self
    }
}



pub struct SSHConnection {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl SSHClient {
    pub fn open_shell(&self) -> std::io::Result<SSHConnection> {
        let mut cmd = Command::new("ssh");

        if self.verbose {
            cmd.arg("-v");
        }
        // cmd.arg("-tt"); 
        cmd.arg("-p").arg(self.port.to_string());
        cmd.arg(format!("{}@{}", self.username, self.destination));

        if let Some(timeout) = self.timeout_secs {
            cmd.arg("-o").arg(format!("ConnectTimeout={}", timeout));
        }

        match &self.auth {
            SSHAuth::PrivateKey { path, .. } => {
                cmd.arg("-i").arg(path);
            }
            SSHAuth::Password(_) => {
                panic!("Password auth not implemented");
            }
        }

        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        Ok(SSHConnection { child, stdin, stdout })
    }
}

impl SSHConnection {
    pub fn exec(&mut self, command: &str) -> std::io::Result<String> {
        const SENTINEL: &str = "__COMMAND_UNIT_DONE__";

        // We echo the sentinel back as a way to determine that a value is returned
        writeln!(self.stdin, "{}; echo {}", command, SENTINEL)?;
        self.stdin.flush()?;

        let mut output = String::new();

        for line in self.stdout.by_ref().lines() {
            let line = line?;
            if line.trim() == SENTINEL {
                break;
            }
            output.push_str(&line);
            output.push('\n');
        }

        Ok(output)
    }
}
