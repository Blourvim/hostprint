use std::{
    env::{self, args},
    io::stdout,
};

use hostprint::{
    commands::{basic, firewall, hardware, package, services},
    connection::{
        common::{exec, ShellTransport},
        local::LocalBash,
        ssh::SSHClient,
    },
    model::host::Host,
    view::md::md::Md,
};
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

fn get_arg_value(flag: &str) -> Option<String> {
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next();
        }
    }
    None
}

fn usage_error(msg: &str) -> std::io::Error {
    eprintln!(
        "{msg}\n\n\
Usage:\n\
  hostprint [--address <host> --port <port> --key <path> --username <user>]\n\n\
If --address is omitted, commands are run locally."
    );

    std::io::Error::new(std::io::ErrorKind::InvalidInput, msg)
}

fn init_logging() {
    let mut verbosity = 0;
    for arg in env::args() {
        if arg == "-v" {
            verbosity += 1;
        } else if arg == "-vv" {
            verbosity += 2;
        } else if arg == "-vvv" {
            verbosity += 3;
        }
    }

    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(
        level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    debug!("Logging initialized at level {:?}", level);
}

enum Client {
    Local,
    Ssh(SSHClient),
}
impl Client {
    pub fn open_shell(&self) -> std::io::Result<Box<dyn ShellTransport>> {
        match self {
            Client::Local => {
                let shell = LocalBash::open()?;
                Ok(Box::new(shell))
            }
            Client::Ssh(ssh) => {
                let shell = ssh.open_shell()?;
                Ok(Box::new(shell))
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    init_logging();

    let address = get_arg_value("--address");
    let client = match address {
        None => {
            info!("Starting hostprint locally");
            Client::Local
        }
        Some(address) => {
            let username =
                get_arg_value("--username").expect("missing --username argument for ssh");
            let port: u16 = get_arg_value("--port")
                .expect("Missing --port argument for SSH")
                .parse()
                .expect("Port must be a number");

            let key = get_arg_value("--key").expect("Missing --key argument for SSH");

            info!("Starting hostprint for {}@{}", username, address);

            let ssh = SSHClient::new(address)
                .with_private_key(key)
                .with_port(port)
                .with_username(username);

            Client::Ssh(ssh)
        }
    };

    let mut host = Host::new();

    let units = vec![
        basic::default_units(),
        package::package_units(),
        firewall::firewall_units(),
        services::running_services_units(),
        hardware::hardware_units(),
    ]
    .concat();

    debug!("Collected {} units to execute", units.len());

    let mut shell = client.open_shell()?;

    for unit in units.iter() {
        info!("Executing unit: {}", unit.name);
        println!("\n=== {} ===", unit.name);

        let stdout = exec(shell.as_mut(), &unit.command)?;
        debug!("Output for {}: {}", unit.name, stdout.trim());

        (unit.follow_up)(&stdout, "", &mut host);
    }

    let md_document = Md::new(&host);
    println!("{}", md_document.content());

    Ok(())
}
