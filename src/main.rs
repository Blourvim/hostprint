use hostprint::{connection::ssh, model::host::Host};

fn main() {
    let my_host: Host = Host::new();
    let mut ssh_command = ssh::SSHClient::new("localhost")
        .with_private_key("./id_rsa")
        .with_command("uname -a")
        .build();

    let res = ssh_command.output().expect("error");
    println!("{:?}",res.stdout);
}
