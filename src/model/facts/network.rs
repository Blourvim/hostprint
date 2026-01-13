use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceFact {
    pub name: String,
    pub flags: Vec<String>,
    pub mtu: Option<u32>,
    pub state: Option<String>,
    pub mac_address: Option<String>,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFacts {
    pub interfaces: Vec<NetworkInterfaceFact>,
}

impl NetworkFacts {
    pub fn from_str(output: &str) -> Self {
        let mut interfaces = Vec::new();
        let mut current_interface: Option<NetworkInterfaceFact> = None;

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // New interface line starts with a number index like "1: lo: ..."
            if let Some(first_char) = line.chars().next() {
                if first_char.is_numeric() && line.contains(": ") {
                    if let Some(interface) = current_interface.take() {
                        interfaces.push(interface);
                    }

                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() >= 2 {
                        let name_part = parts[1]; // "lo" or "ens4"
                        let meta_part = if parts.len() > 2 { parts[2] } else { "" };

                        // Parse flags inside <...>
                        let mut flags = Vec::new();
                        if let Some(start) = meta_part.find('<') {
                            if let Some(end) = meta_part.find('>') {
                                flags = meta_part[start + 1..end]
                                    .split(',')
                                    .map(|s| s.to_string())
                                    .collect();
                            }
                        }

                        // Parse mtu, state
                        let mut mtu = None;
                        let mut state = None;
                        let meta_tokens: Vec<&str> = meta_part.split_whitespace().collect();
                        for (i, token) in meta_tokens.iter().enumerate() {
                            if *token == "mtu" && i + 1 < meta_tokens.len() {
                                mtu = meta_tokens[i + 1].parse().ok();
                            }
                            if *token == "state" && i + 1 < meta_tokens.len() {
                                state = Some(meta_tokens[i + 1].to_string());
                            }
                        }

                        current_interface = Some(NetworkInterfaceFact {
                            name: name_part.to_string(),
                            flags,
                            mtu,
                            state,
                            mac_address: None,
                            ipv4: Vec::new(),
                            ipv6: Vec::new(),
                        });
                    }
                    continue;
                }
            }

            // Parse attributes for current interface
            if let Some(ref mut interface) = current_interface {
                let tokens: Vec<&str> = line.split_whitespace().collect();
                if tokens.is_empty() {
                    continue;
                }

                match tokens[0] {
                    "link/ether" | "link/loopback" => {
                        if tokens.len() > 1 {
                            interface.mac_address = Some(tokens[1].to_string());
                        }
                    }
                    "inet" => {
                        if tokens.len() > 1 {
                            interface.ipv4.push(tokens[1].to_string());
                        }
                    }
                    "inet6" => {
                        if tokens.len() > 1 {
                            interface.ipv6.push(tokens[1].to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(interface) = current_interface {
            interfaces.push(interface);
        }

        NetworkFacts { interfaces }
    }
}
