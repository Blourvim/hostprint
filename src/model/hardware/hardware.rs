#[derive(Debug)]
pub struct Hardware {
    // CPU
    cpu_architecture: Option<String>,
    cpu_model: Option<String>,
    cpu_vendor: Option<String>,
    cpu_cores: Option<u32>,
    cpu_threads: Option<u32>,
    cpu_frequency_mhz: Option<f64>,
    cpu_flags: Option<Vec<String>>,

    // Memory
    memory_total_kb: Option<u64>,
    memory_free_kb: Option<u64>,
    memory_available_kb: Option<u64>,
    swap_total_kb: Option<u64>,
    swap_free_kb: Option<u64>,

    // Storage
    disks: Option<Vec<DiskInfo>>,
    partitions: Option<Vec<PartitionInfo>>,

    // Motherboard / BIOS
    motherboard_vendor: Option<String>,
    motherboard_model: Option<String>,
    bios_vendor: Option<String>,
    bios_version: Option<String>,
    bios_date: Option<String>,

    // GPU
    gpus: Option<Vec<GpuInfo>>,

    // Network
    network_interfaces: Option<Vec<NetworkInterface>>,

    uptime_seconds: Option<u64>,
    battery_capacity_percent: Option<f32>,
    battery_status: Option<String>,
}

#[derive(Debug)]
struct DiskInfo {
    name: String,
    model: Option<String>,
    size_bytes: Option<u64>,
    rotational: Option<bool>, // SSD = false, HDD = true
}

#[derive(Debug)]
struct PartitionInfo {
    name: String,
    mount_point: Option<String>,
    filesystem: Option<String>,
    size_bytes: Option<u64>,
    used_bytes: Option<u64>,
}

#[derive(Debug)]
struct GpuInfo {
    name: Option<String>,
    vendor: Option<String>,
    memory_mb: Option<u64>,
    driver: Option<String>,
}

#[derive(Debug)]
struct NetworkInterface {
    name: String,
    mac_address: Option<String>,
    ipv4_addresses: Option<Vec<String>>,
    ipv6_addresses: Option<Vec<String>>,
    speed_mbps: Option<u64>,
    is_up: Option<bool>,
}
