use hostprint::{connection::ssh, model::host::Host};

fn main() {
    let my_host: Host = Host::new();
    let mut ssh_command = ssh::SSHClient::new("127.0.0.1")
        .with_private_key("./id_rsa")
        .with_command("uname -a")
        .with_port(3000u16)
        .with_username("devel")
        .build();

    let res = ssh_command.output().expect("error");
    println!("stdout{:?}",str::from_utf8(&res.stdout).unwrap());
    println!("status{:?}",res.status);
    println!("stderr{:?}",str::from_utf8(&res.stderr).unwrap());
}
