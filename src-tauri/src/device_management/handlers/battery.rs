use serde::Serialize;

use rsmobiledevice::{device::DeviceClient, devices_collection::SingleDevice, RecursiveFind};

#[derive(Serialize, Clone)]
pub struct Battery {
    pub battery_level: u8,
    pub battery_health: f32,
    pub cycle_counts: u32,
}

pub fn handle_device_battery(device: &DeviceClient<SingleDevice>) -> Battery {
    let device_diag = device.get_device_diagnostic();

    let battery_plist = device_diag.get_battery_plist().unwrap();

    let battery_level = battery_plist
        .rfind("CurrentCapacity")
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let cycle_counts = battery_plist
        .rfind("CycleCount")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let _designed_cap = battery_plist
        .rfind("DesignCapacity")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let _max_cap = battery_plist
        .rfind("NominalChargeCapacity")
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let battery_health = ((_max_cap / _designed_cap) * 100.0 * 100.0).round() / 100.0;

    Battery {
        battery_level,
        battery_health,
        cycle_counts,
    }
}
