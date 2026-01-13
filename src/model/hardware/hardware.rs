#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct Hardware {
    // CPU
    pub cpu_architecture: Option<String>,
    pub cpu_model: Option<String>,
    pub cpu_vendor: Option<String>,
    pub cpu_cores: Option<u32>,
    pub cpu_threads: Option<u32>,
    pub cpu_frequency_mhz: Option<f64>,
    pub cpu_flags: Option<Vec<String>>,

    // Memory
    pub memory_total_kb: Option<u64>,
    pub memory_free_kb: Option<u64>,
    pub memory_available_kb: Option<u64>,
    pub swap_total_kb: Option<u64>,
    pub swap_free_kb: Option<u64>,

    // Storage
    pub disks: Option<Vec<DiskInfo>>,
    pub partitions: Option<Vec<PartitionInfo>>,

    // Motherboard / BIOS
    pub motherboard_vendor: Option<String>,
    pub motherboard_model: Option<String>,
    pub bios_vendor: Option<String>,
    pub bios_version: Option<String>,
    pub bios_date: Option<String>,

    // GPU
    pub gpus: Option<Vec<GpuInfo>>,

    // Network
    pub network_interfaces: Option<Vec<NetworkInterface>>,

    pub uptime_seconds: Option<u64>,
    pub battery_capacity_percent: Option<f32>,
    pub battery_status: Option<String>,
}

#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub model: Option<String>,
    pub size_bytes: Option<u64>,
    pub rotational: Option<bool>, // SSD = false, HDD = true
}

#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct PartitionInfo {
    pub name: String,
    pub mount_point: Option<String>,
    pub filesystem: Option<String>,
    pub size_bytes: Option<u64>,
    pub used_bytes: Option<u64>,
}

#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct GpuInfo {
    pub name: Option<String>,
    pub vendor: Option<String>,
    pub memory_mb: Option<u64>,
    pub driver: Option<String>,
}

#[derive(serde::Deserialize,serde::Serialize,Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: Option<String>,
    pub ipv4_addresses: Option<Vec<String>>,
    pub ipv6_addresses: Option<Vec<String>>,
    pub speed_mbps: Option<u64>,
    pub is_up: Option<bool>,
}
