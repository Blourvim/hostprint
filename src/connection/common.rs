use std::io::{BufRead, Write};

/// A bidirectional shell transport.
///
/// Anything that can send bytes to a shell-like process
/// and read line-oriented output back can implement this.
pub trait ShellTransport {
    fn stdin(&mut self) -> &mut dyn Write;
    fn stdout(&mut self) -> &mut dyn BufRead;
}

/// Execute a command over a shell transport and return its stdout.
///
/// Uses a sentinel value to determine when the command has completed.
pub fn exec(transport: &mut dyn ShellTransport, command: &str) -> std::io::Result<String> {
    const SENTINEL: &str = "__COMMAND_UNIT_DONE__";

    log::debug!("Executing command: {}", command);

    // Send command + sentinel
    writeln!(transport.stdin(), "{}; echo {}", command, SENTINEL)?;
    transport.stdin().flush()?;

    let mut output = String::new();

    for line in transport.stdout().lines() {
        let line = line?;
        if line.trim() == SENTINEL {
            break;
        }
        output.push_str(&line);
        output.push('\n');
    }

    log::trace!("Command output received (len: {})", output.len());

    Ok(output)
}
