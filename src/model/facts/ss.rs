use std::str;

#[derive(Debug, Clone)]
pub struct SsFacts {
    pub entries: Vec<SsEntry>,
}

#[derive(Debug, Clone)]
pub struct SsEntry {
    pub protocol: String,
    pub state: String,
    pub recv_q: u64,
    pub send_q: u64,
    pub local_address: String,
    pub peer_address: String,
    pub process: Option<String>,
}

impl SsFacts {
    pub fn from_std(output: &str) -> SsFacts {
        let entries: Vec<SsEntry> = output
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|line| SsEntry::from_line(line).ok())
            .collect();

        SsFacts { entries }
    }

    pub fn from_entries(entries: Vec<SsEntry>) -> SsFacts {
        SsFacts { entries }
    }
}

impl SsEntry {
    pub fn from_line(line: &str) -> Result<Self, String> {
        // Split into at most 7 parts, to preserve the process field (which may contain spaces)
        let mut parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 6 {
            return Err(format!("Malformed ss line: {}", line));
        }

        let protocol = parts[0].to_string();
        let state = parts[1].to_string();
        let recv_q = parts[2].parse::<u64>().unwrap_or(0);
        let send_q = parts[3].parse::<u64>().unwrap_or(0);

        // The last two are always local and peer addresses
        let local_address = parts[parts.len() - 2].to_string();
        let peer_address = parts[parts.len() - 1].to_string();

        // Check if a process section exists (it usually starts with "users:")
        // Everything between "users:" and the end of the line should be captured.
        let process = if let Some(idx) = line.find("users:") {
            Some(line[idx..].trim().to_string())
        } else {
            None
        };

        Ok(SsEntry {
            protocol,
            state,
            recv_q,
            send_q,
            local_address,
            peer_address,
            process,
        })
    }

    pub fn new(
        protocol: String,
        state: String,
        recv_q: u64,
        send_q: u64,
        local_address: String,
        peer_address: String,
        process: Option<String>,
    ) -> Self {
        SsEntry {
            protocol,
            state,
            recv_q,
            send_q,
            local_address,
            peer_address,
            process,
        }
    }
}
