use std::env;
use crate::model::host::Host;

pub fn noop_follow_up(stdout: &str, stderr: &str, _host: &mut Host) {
    // Check if "--verbose" or "-v" exists in the command-line arguments
    let verbose = env::args().any(|arg| arg == "--verbose" || arg == "-v");

    if verbose {
        println!("Command output:\n{}", stdout);
        if !stderr.trim().is_empty() {
            eprintln!("stderr: {}", stderr);
        }
    }
}
