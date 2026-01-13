#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Tcp6,
    Udp6,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Process {
    pub pid: Option<u32>, 
    pub uid: Option<u32>,
    pub gid: Option<u32>, 
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Socket {
    pub protocol: Protocol,

    pub address: String,
    pub port: u16,

    // Remote address + port (None for LISTEN sockets)
    pub remote_address: Option<String>,
    pub remote_port: Option<u16>,

    // Kernel socket state (LISTEN, ESTABLISHED, etc.)
    pub state: String,

    // OS process ownership fields
    pub process: Process,

    // System-level socket identifiers
    pub inode: u64,    // Socket inode from /proc/net/*
    pub rx_queue: u32, // Receive queue size
    pub tx_queue: u32, // Send queue size
    pub flags: u32,    // Kernel socket flags

    // Timer data from /proc/net/tcp
    pub timer_active: u8,  // Timer type (0–5 depending on kernel)
    pub timer_expire: u64, // Microseconds until expiration
    pub retransmits: u8,   // Number of retransmissions

    // Which network interface this socket is bound to (if detectable)
    pub interface: Option<String>,
}
