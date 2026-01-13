use crate::model::{
    facts::network::NetworkFacts,
    hardware::hardware::{Hardware, NetworkInterface},
    host::Host,
};

pub fn network_interfaces_follow_up(stdout: &str, _stderr: &str, host: &mut Host) {
    let facts = NetworkFacts::from_str(stdout);

    let mut interfaces = Vec::new();
    for fact in facts.interfaces {
        interfaces.push(NetworkInterface {
            name: fact.name,
            mac_address: fact.mac_address,
            ipv4_addresses: Some(fact.ipv4),
            ipv6_addresses: Some(fact.ipv6),
            speed_mbps: None, // Not available from ip addr
            is_up: Some(fact.state.as_deref() == Some("UP")),
        });
    }

    if let Some(hardware) = &mut host.hardware {
        hardware.network_interfaces = Some(interfaces);
    } else {
        // Create new hardware struct if it doesn't exist
        // Since Hardware has many fields, we should probably use a default or builder if available.
        // But Hardware doesn't derive Default.
        // I'll just initialize with None.
        let hardware = Hardware {
            cpu_architecture: None,
            cpu_model: None,
            cpu_vendor: None,
            cpu_cores: None,
            cpu_threads: None,
            cpu_frequency_mhz: None,
            cpu_flags: None,
            memory_total_kb: None,
            memory_free_kb: None,
            memory_available_kb: None,
            swap_total_kb: None,
            swap_free_kb: None,
            disks: None,
            partitions: None,
            motherboard_vendor: None,
            motherboard_model: None,
            bios_vendor: None,
            bios_version: None,
            bios_date: None,
            gpus: None,
            network_interfaces: Some(interfaces),
            uptime_seconds: None,
            battery_capacity_percent: None,
            battery_status: None,
        };
        host.hardware = Some(hardware);
    }
}
