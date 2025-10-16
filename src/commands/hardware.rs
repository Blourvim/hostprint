use crate::commands::common::noop::noop_follow_up;
use crate::commands::unit::Unit;

pub fn hardware_units() -> Vec<Unit> {
    vec![
        // CPU
        Unit::new("CPU Info", "lscpu", noop_follow_up),
        // RAM
        Unit::new("Memory Info", "free -h", noop_follow_up),
        Unit::new("Memory Details", "cat /proc/meminfo", noop_follow_up),
        // GPU
        Unit::new(
            "GPU Info",
            "lspci | grep -i 'vga\\|3d\\|gpu'",
            noop_follow_up,
        ),
        // Storage / Disks
        Unit::new(
            "Block Devices",
            "lsblk -o NAME,SIZE,TYPE,MOUNTPOINT",
            noop_follow_up,
        ),
        Unit::new("Disk Usage", "df -h", noop_follow_up),
        // Peripherals
        Unit::new("USB Devices", "lsusb", noop_follow_up),
        Unit::new("PCI Devices", "lspci", noop_follow_up),
        // Motherboard (limited without sudo)
        Unit::new(
            "Motherboard Info (limited)",
            "cat /sys/devices/virtual/dmi/id/board_{vendor,name,version,serial}",
            noop_follow_up,
        ),
    ]
}
