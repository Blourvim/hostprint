use crate::commands::{
    follow_up::basic::{df_follow_up, du_follow_up},
    unit::Unit,
};

pub fn units() -> Vec<Unit> {
    return vec![
        Unit::new("Disk Usage", "df", df_follow_up),
        Unit::new(
            "Largest Directories",
            "du -sh --time /* 2>/dev/null",
            du_follow_up,
        ),
    ];
}
