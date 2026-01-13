use crate::model::{
    facts::memory::MemoryFacts,
    host::Host,
    hardware::hardware::Hardware,
};

fn parse_size(s: &str) -> Option<u64> {
    // Parse "1.9Gi" or "46Mi" to KB
    // This is a simplified parser, assuming standard suffixes
    let s = s.trim();
    let len = s.len();
    if len < 2 { return None; }
    
    let (num_str, suffix) = s.split_at(len - 2);
    let num: f64 = num_str.parse().ok()?;
    
    let multiplier = match suffix {
        "Ki" => 1.0,
        "Mi" => 1024.0,
        "Gi" => 1024.0 * 1024.0,
        "Ti" => 1024.0 * 1024.0 * 1024.0,
        _ => return None, // Handle B or others if needed
    };
    
    Some((num * multiplier) as u64)
}

pub fn memory_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = MemoryFacts::from_str(stdout);
    
    let total = facts.total.as_ref().and_then(|s| parse_size(s));
    let free = facts.free.as_ref().and_then(|s| parse_size(s));
    let available = facts.available.as_ref().and_then(|s| parse_size(s));
    let swap_total = facts.swap_total.as_ref().and_then(|s| parse_size(s));
    let swap_free = facts.swap_free.as_ref().and_then(|s| parse_size(s));

    if let Some(hardware) = &mut host.hardware {
        hardware.memory_total_kb = total;
        hardware.memory_free_kb = free;
        hardware.memory_available_kb = available;
        hardware.swap_total_kb = swap_total;
        hardware.swap_free_kb = swap_free;
    } else {
         let mut hardware = Hardware {
            cpu_architecture: None,
            cpu_model: None,
            cpu_vendor: None,
            cpu_cores: None,
            cpu_threads: None,
            cpu_frequency_mhz: None,
            cpu_flags: None,
            memory_total_kb: total,
            memory_free_kb: free,
            memory_available_kb: available,
            swap_total_kb: swap_total,
            swap_free_kb: swap_free,
            disks: None,
            partitions: None,
            motherboard_vendor: None,
            motherboard_model: None,
            bios_vendor: None,
            bios_version: None,
            bios_date: None,
            gpus: None,
            network_interfaces: None,
            uptime_seconds: None,
            battery_capacity_percent: None,
            battery_status: None,
        };
        host.hardware = Some(hardware);
    }
}
