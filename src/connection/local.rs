use std::io::BufReader;
use std::process::{Child, Command, Stdio};

use crate::connection::common::ShellTransport;

pub struct LocalBash {
    child: Child,
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
}


impl LocalBash {
    pub fn open() -> std::io::Result<Self> {
        let mut child = Command::new("bash")
            // Keep output deterministic
            .arg("--noprofile")
            .arg("--norc")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child.stdin.take().expect("stdin not captured");
        let stdout = BufReader::new(child.stdout.take().expect("stdout not captured"));

        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }

    pub fn child(&self) -> &Child {
        &self.child
    }
}

impl ShellTransport for LocalBash {
    fn stdin(&mut self) -> &mut dyn std::io::Write {
        &mut self.stdin
    }

    fn stdout(&mut self) -> &mut dyn std::io::BufRead {
        &mut self.stdout
    }
}
