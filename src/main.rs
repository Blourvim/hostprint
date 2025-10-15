use hostprint::{commands::basic, connection::ssh, model::host::Host};

fn main() {
    let mut my_host: Host = Host::new();
    let ssh_command = ssh::SSHClient::new("127.0.0.1")
        .with_private_key("./src/connection/keys/id_rsa")
        .with_command("uname -a")
        .with_port(3000u16)
        .with_username("devel");

for unit in basic::default_units().iter() {
        println!("\n=== Running : {} ===", unit.name);

        // Clone the base SSH cofig and set its command
        let mut ssh_command = ssh_command.clone().with_command(&unit.comand).build();

        let res = ssh_command.output().expect("failed to execute SSH command");

        let stdout = String::from_utf8_lossy(&res.stdout);
        let stderr = String::from_utf8_lossy(&res.stderr);
        (unit.follow_up)(&stdout, &stderr, &mut my_host);
    }
}
