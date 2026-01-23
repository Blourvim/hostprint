use std::fmt::Write;

use crate::model::network::socket::Socket;

pub fn generate_sockets_section(content: &mut String, sockets: &Option<Vec<Socket>>) {
    writeln!(content, "## Network Sockets").unwrap();

    let Some(sockets) = sockets else {
        writeln!(content, "No socket information available").unwrap();
        return;
    };

    if sockets.is_empty() {
        writeln!(content, "No socket information available").unwrap();
        return;
    }

    writeln!(
        content,
        "| Proto | Local Address | Remote Address | State | PID | UID | GID | Interface |"
    )
    .unwrap();
    writeln!(
        content,
        "|-------|---------------|----------------|-------|-----|-----|-----|-----------|"
    )
    .unwrap();

    for s in sockets {
        let local = format!("{}:{}", s.address, s.port);

        let remote = match (&s.remote_address, s.remote_port) {
            (Some(addr), Some(port)) => format!("{}:{}", addr, port),
            _ => "N/A".to_string(),
        };

        writeln!(
            content,
            "| {:?} | {} | {} | {} | {} | {} | {} | {} |",
            s.protocol,
            local,
            remote,
            s.state,
            s.process.pid.map(|v| v.to_string()).unwrap_or("N/A".into()),
            s.process.uid.map(|v| v.to_string()).unwrap_or("N/A".into()),
            s.process.gid.map(|v| v.to_string()).unwrap_or("N/A".into()),
            s.interface.as_deref().unwrap_or("N/A"),
        )
        .unwrap();
    }
}

#[cfg(test)]
mod generate_sockets_tests {
    use crate::model::network::socket::{Process, Protocol};

    use super::*;

    #[test]
    fn renders_no_sockets_none() {
        let mut content = String::new();

        generate_sockets_section(&mut content, &None);

        assert!(content.contains("## Network Sockets"));
        assert!(content.contains("No socket information available"));
    }

    #[test]
    fn renders_no_sockets_empty() {
        let mut content = String::new();

        generate_sockets_section(&mut content, &Some(vec![]));

        assert!(content.contains("## Network Sockets"));
        assert!(content.contains("No socket information available"));
    }

    #[test]
    fn renders_socket_entry() {
        let mut content = String::new();

        let sockets = vec![Socket {
            protocol: Protocol::Tcp,
            address: "127.0.0.1".into(),
            port: 8080,
            remote_address: Some("192.168.1.10".into()),
            remote_port: Some(52344),
            state: "ESTABLISHED".into(),
            process: Process {
                pid: Some(1234),
                uid: Some(1000),
                gid: Some(1000),
            },
            inode: 42,
            rx_queue: 0,
            tx_queue: 0,
            flags: 0,
            timer_active: 0,
            timer_expire: 0,
            retransmits: 0,
            interface: Some("eth0".into()),
        }];

        generate_sockets_section(&mut content, &Some(sockets));

        assert!(content.contains(
            "| Proto | Local Address | Remote Address | State | PID | UID | GID | Interface |"
        ));
        assert!(content.contains("| Tcp | 127.0.0.1:8080 | 192.168.1.10:52344 | ESTABLISHED | 1234 | 1000 | 1000 | eth0 |"));
    }

    #[test]
    fn renders_listen_socket_without_remote() {
        let mut content = String::new();

        let sockets = vec![Socket {
            protocol: Protocol::Tcp,
            address: "0.0.0.0".into(),
            port: 22,
            remote_address: None,
            remote_port: None,
            state: "LISTEN".into(),
            process: Process {
                pid: None,
                uid: None,
                gid: None,
            },
            inode: 1,
            rx_queue: 0,
            tx_queue: 0,
            flags: 0,
            timer_active: 0,
            timer_expire: 0,
            retransmits: 0,
            interface: None,
        }];

        generate_sockets_section(&mut content, &Some(sockets));

        assert!(content.contains("| LISTEN |"));
        assert!(content.contains("| N/A |"));
        assert!(content.contains("| 0 | 0 | 0 | N/A |"));
    }
}
