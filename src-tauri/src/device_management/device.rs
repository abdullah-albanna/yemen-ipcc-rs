use rsmobiledevice::device_info::domains::DeviceDomains;
use rsmobiledevice::device_info::keys::DeviceKeys;
use std::sync::Mutex;
use std::time::Duration;
use std::{sync::Arc, thread};
use tauri::Emitter;

use super::handlers::{
    battery::handle_device_battery, hardware::handle_device_hardware, os::handle_device_os,
    storage::handle_device_storage,
};

#[tauri::command]
pub fn install_ipcc(window: tauri::Window, device_model: String, ios_ver: String) {
    if let Ok(device_clients) = rsmobiledevice::device::DeviceClient::new() {
        if let Some(device_client) = device_clients.get_first_device() {
            let device_info = device_client.get_device_info();

            let c_model = device_info
                .get_value(DeviceKeys::ProductType, DeviceDomains::All)
                .unwrap_or_default();
            let c_ios_ver = device_info
                .get_value(DeviceKeys::ProductVersion, DeviceDomains::All)
                .unwrap_or_default();

            if device_model != c_model || ios_ver != c_ios_ver {
                window.emit("installing_error", true).unwrap_or_default();
                return;
            }

            let _install_client = device_client.get_device_installer();

            let _ipcc_file = "get the file from the api";
        }
    }
}

// Device monitoring function
#[tauri::command]
pub fn check_device(window: tauri::Window) {
    let device_disconnected = Arc::new(Mutex::new(false));

    // we start out with true to send the device info if it's connnected
    //
    // if it was false then we will only send the device info if it got disconnected then
    // reconnected, we don't want that
    let status_changed = Arc::new(Mutex::new(true));

    thread::spawn({
        move || loop {
            if let Ok(device) = rsmobiledevice::device::DeviceClient::new() {
                let Some(device) = device.get_first_device() else {
                    continue;
                };

                if let (Ok(mut status_c), Ok(mut device_disconnected_value)) =
                    (status_changed.try_lock(), device_disconnected.try_lock())
                {
                    // only if the status changed we send it again
                    if *status_c {
                        let hardware = handle_device_hardware(&device);
                        let storage = handle_device_storage(&device);
                        let battery = handle_device_battery(&device);
                        let os = handle_device_os(&device);

                        // Emit connected status with device info
                        window.emit("device_hardware", hardware).unwrap();
                        window.emit("device_storage", storage).unwrap();
                        window.emit("device_battery", battery).unwrap();
                        window.emit("device_os", os).unwrap();

                        // we set the disconnected value to false indicating it is connnected
                        *device_disconnected_value = false;
                        *status_c = false;
                        window.emit("device_status", true).unwrap();
                    }
                }
            } else if let (Ok(mut device_disconnected_value), Ok(mut status_changed_value)) =
                (device_disconnected.try_lock(), status_changed.try_lock())
            {
                // if the value is false, meaning it was connected, and now it got disconnected
                if !(*device_disconnected_value) {
                    window.emit("device_status", false).unwrap();
                    // we set the disconnected value to true indicating that it is disconnected
                    //
                    // also we put the status changed to true, because it got disconnected
                    // even if it was disconnected from the beginning, we will only assign it back
                    // to true once
                    *device_disconnected_value = true;
                    *status_changed_value = true;
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}
