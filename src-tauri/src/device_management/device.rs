use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Emitter;

use super::handlers::{
    battery::handle_device_battery, hardware::handle_device_hardware, os::handle_device_os,
    storage::handle_device_storage,
};

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
