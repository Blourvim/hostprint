use crate::commands::common::noop::noop_follow_up;
use crate::commands::unit::Unit;

pub fn running_services_units() -> Vec<Unit> {
    vec![
        Unit::new(
            "Active Services",
            "systemctl list-units --type=service --state=running",
            noop_follow_up,
        ),
        Unit::new(
            "Active Services Count",
            "systemctl list-units --type=service --state=running | wc -l",
            noop_follow_up,
        ),
        Unit::new(
            "Failed Services",
            "systemctl --failed --type=service",
            noop_follow_up,
        ),
        Unit::new(
            "Top Services by Memory",
            "systemctl list-units --type=service --state=running --no-pager --all | head -20",
            noop_follow_up,
        ),
    ]
}
