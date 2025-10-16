use std::env;

use hostprint::{
    commands::{basic, firewall, hardware, package, services},
    connection::ssh::SSHClient,
    model::host::Host,
};
fn get_arg_value(flag: &str) -> Option<String> {
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == flag {
            return args.next();
        }
    }
    None
}
fn main() -> std::io::Result<()> {
    let address = get_arg_value("--address").expect("Missing --address argument");
    let port: u16 = get_arg_value("--port")
        .expect("Missing --port argument")
        .parse()
        .expect("Port must be a number");
    let key = get_arg_value("--key").expect("Missing --key argument");
    let username = get_arg_value("--username").unwrap_or("".to_string());
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
    let mut shell = client.open_shell()?;

    for unit in units.iter() {
        println!("\n=== {} ===", unit.name);
        let stdout = shell.exec(&unit.comand)?;
        (unit.follow_up)(&stdout, "", &mut host);
    }
    Ok(())
}
