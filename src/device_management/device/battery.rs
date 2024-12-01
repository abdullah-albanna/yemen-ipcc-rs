use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct DeviceBattery {
    battery_level: u8,
    battery_health: u8,
    cycle_counts: u32,
}

impl Display for DeviceBattery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
        Battery Level: {}%
        Battery Health: {}%,
        Cycle Counts: {}
        "#,
            self.battery_level, self.battery_health, self.cycle_counts
        )
    }
}

impl DeviceBattery {
    pub fn new(battery_level: u8, battery_health: u8, cycle_counts: u32) -> Self {
        Self {
            battery_level,
            battery_health,
            cycle_counts,
        }
    }
    pub fn get_battery_level(&self) -> String {
        format!("{}%", self.battery_level)
    }

    pub fn get_battery_health(&self) -> String {
        format!("{}%", self.battery_health)
    }

    pub fn get_cycle_counts(&self) -> String {
        self.cycle_counts.to_string()
    }
}
