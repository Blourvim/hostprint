use crate::{commands::{common::noop::noop_follow_up, unit::Unit}};


pub fn firewall_units() -> Vec<Unit> {
    vec![
        Unit::new(
            "Firewall Services",
            "systemctl list-units --type=service --all | grep -E 'ufw|firewalld|nftables'",
            noop_follow_up,
        ),

        Unit::new(
            "firewalld Active",
            "systemctl is-active firewalld 2>/dev/null || echo inactive",
            noop_follow_up,
        ),

        Unit::new(
            "ufw Active",
            "systemctl is-active ufw 2>/dev/null || echo inactive",
            noop_follow_up,
        ),

        Unit::new(
            "nftables Active",
            "systemctl is-active nftables 2>/dev/null || echo inactive",
            noop_follow_up,
        ),
    ]
}
