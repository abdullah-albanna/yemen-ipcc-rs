use rsmobiledevice::device_info::domains::DeviceDomains;
use rsmobiledevice::device_info::keys::DeviceKeys;
use rsmobiledevice::devices_collection::SingleDevice;
use rsmobiledevice::RecursiveFind;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Emitter;

#[derive(Serialize, Clone)]
pub struct Hardware {
    pub model: String,
    pub model_number: String,
    pub region: String,
}

#[derive(Serialize, Clone)]
pub struct OS {
    pub ios_ver: String,
    pub build_num: String,
}

#[derive(Serialize, Clone)]
pub struct Battery {
    pub battery_level: u8,
    pub battery_health: f32,
    pub cycle_counts: u32,
}

#[derive(Serialize, Clone)]
pub struct Storage {
    pub total_storage: u64,
    pub used_storage: u64,
    pub available_storage: u64,
}

// Function to handle a connected device
pub fn handle_device_hardware(
    device: &rsmobiledevice::device::DeviceClient<SingleDevice>,
) -> Hardware {
    let device_info = device.get_device_info();

    let region_code = device_info
        .get_value(DeviceKeys::RegionInfo, DeviceDomains::All)
        .unwrap_or("unknown".into())
        .trim()
        .to_owned();

    let region: String = match region_code.as_str() {
        "LL/A" => "United States".into(),
        "B/A" => "Canada".into(),
        "C/A" => "Europe".into(),
        "J/A" => "Japan".into(),
        "X/A" => "China".into(),
        "ZP/A" => "Global/International".into(),
        "KH/A" => "Hong Kong".into(),
        "M/A" => "Mexico".into(),
        "A/A" => "Argentina".into(),
        "T/A" => "Taiwan".into(),
        "V/A" => "United Kingdom".into(),
        "R/A" => "Russia".into(),
        _ => "unknown".into(),
    };

    let model_number_code = device_info
        .get_value(DeviceKeys::ModelNumber, DeviceDomains::All)
        .unwrap_or("unknown".into());

    let model_meaning = match model_number_code.chars().next().unwrap_or_default() {
        'F' => "Refurbished Device",
        'M' => "New Device",
        'N' => "Warranty Replacement Device",
        'P' => "Personalized Device",
        '3' => "Demo Device",
        _ => "unknown",
    };

    let model_number = format!("{} ({})", model_number_code, model_meaning);

    Hardware {
        model: device_info.get_product_type(),
        model_number,
        region,
    }
}

pub fn handle_device_battery(
    device: &rsmobiledevice::device::DeviceClient<SingleDevice>,
) -> Battery {
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

pub fn handle_device_storage(
    device: &rsmobiledevice::device::DeviceClient<SingleDevice>,
) -> Storage {
    let device_info = device.get_device_info();

    let disk_dict = device_info.get_values(DeviceDomains::DiskUsage).unwrap();
    let total_storage = disk_dict
        .get("TotalDiskCapacity")
        .unwrap()
        .to_owned()
        .parse::<u64>()
        .unwrap();
    let available_storage = disk_dict
        .get("AmountRestoreAvailable")
        .unwrap()
        .to_owned()
        .parse::<u64>()
        .unwrap();
    let used_storage = total_storage - available_storage;

    Storage {
        total_storage,
        available_storage,
        used_storage,
    }
}

pub fn handle_device_os(device: &rsmobiledevice::device::DeviceClient<SingleDevice>) -> OS {
    let device_info = device.get_device_info();

    OS {
        ios_ver: device_info
            .get_value(DeviceKeys::ProductVersion, DeviceDomains::All)
            .unwrap(),
        build_num: device_info
            .get_value(DeviceKeys::BuildVersion, DeviceDomains::All)
            .unwrap(),
    }
}

// Device monitoring function
#[tauri::command]
pub fn check_device(window: tauri::Window) {
    let emitted_device_status = Arc::new(Mutex::new(false)); // Track if a "disconnected" message has already been sent

    thread::spawn({
        let emitted_device_status = emitted_device_status.clone();
        move || loop {
            if let Ok(device) = rsmobiledevice::device::DeviceClient::new() {
                let device = device.get_first_device();

                let hardware = handle_device_hardware(&device);
                let storage = handle_device_storage(&device);
                let battery = handle_device_battery(&device);
                let os = handle_device_os(&device);

                // Emit connected status with device info
                window.emit("device_hardware", hardware).unwrap();
                window.emit("device_storage", storage).unwrap();
                window.emit("device_battery", battery).unwrap();
                window.emit("device_os", os).unwrap();
                *emitted_device_status.lock().unwrap() = false; // Reset disconnected flag
            } else {
                let mut emitted = emitted_device_status.lock().unwrap();
                if !*emitted {
                    // Emit disconnected status only once
                    window.emit("device_status", false).unwrap();
                    *emitted = true;
                }
            }
            thread::sleep(Duration::from_secs(2)); // Check every 2 seconds
        }
    });
}
