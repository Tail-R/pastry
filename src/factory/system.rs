use crate::factory::{
    shell::poll_label,
};

pub fn bat_label(name: &str) -> gtk::Label {
    poll_label(name, "cat /sys/class/power_supply/BAT0/capacity", 60 as u64)
}
