use std::env;

use hostprint::{
    commands::{basic, firewall, hardware, package, services},
    connection::ssh::SSHClient,
    model::host::Host,
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

fn main() -> std::io::Result<()> {
    init_logging();

    let address = get_arg_value("--address").expect("Missing --address argument");
    let port: u16 = get_arg_value("--port")
        .expect("Missing --port argument")
        .parse()
        .expect("Port must be a number");
    let key = get_arg_value("--key").expect("Missing --key argument");
    let username = get_arg_value("--username").unwrap_or("".to_string());

    info!("Starting hostprint for {}@{}", username, address);

    let mut host = Host::new();

    let client = SSHClient::new(address)
        .with_private_key(key)
        .with_port(port)
        .with_username(username);

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
        let stdout = shell.exec(&unit.command)?;
        debug!("Output for {}: {}", unit.name, stdout.trim());
        (unit.follow_up)(&stdout, "", &mut host);
    }

    Ok(())
}
