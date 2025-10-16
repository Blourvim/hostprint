use hostprint::{
    commands::{basic, firewall, package, services},
    connection::ssh::SSHClient,
    model::host::Host,
};

fn main() -> std::io::Result<()> {
    let mut host = Host::new();

    let client = SSHClient::new("127.0.0.1")
        .with_private_key("./id_rsa")
        .with_port(3000u16)
        .with_username("devel");

    let mut shell = client.open_shell()?;
    // let units = basic::default_units();
    // let units = package::package_units();
    // let units = firewall::firewall_units();
    let units = services::running_services_units();
    for unit in units.iter() {
        println!("\n=== {} ===", unit.name);
        let stdout = shell.exec(&unit.comand)?;
        (unit.follow_up)(&stdout, "", &mut host);
    }
    Ok(())
}
