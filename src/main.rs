use std::{env, net::TcpListener};

use hostprint::{
    commands::{basic, firewall, hardware, package, services},
    connection::ssh::SSHClient,
    model::host::Host,
};
use std::io::{BufRead, Write};
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
        let stdout = shell.exec(&unit.command)?;
        (unit.follow_up)(&stdout, "", &mut host);
    }

    if let Some(serve_port) = get_arg_value("--serve") {

        const PAGE: &str = include_str!("./view/template/index.html");
        let listener = std::net::TcpListener::bind(&serve_port).unwrap();

        println!("Dashboard is available at{:?}", &serve_port);
        for mut stream in listener.incoming().flatten() {
            let mut reader = std::io::BufReader::new(&mut stream);
            let mut l = String::new();
            reader.read_line(&mut l).unwrap();
            stream.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
            stream
                .write_all(b"Content-Type: text/html; charset=UTF-8\r\n")
                .unwrap();
            stream.write_all(b"\r\n").unwrap();
            stream.write_all(PAGE.as_bytes()).unwrap();
        }
    }
    Ok(())
}
