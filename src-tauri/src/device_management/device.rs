use regex::Regex;
use rsmobiledevice::device_info::domains::DeviceDomains;
use rsmobiledevice::device_info::keys::DeviceKeys;
use rsmobiledevice::device_syslog::filters::FilterPart;
use rsmobiledevice::device_syslog::LogFilter;
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
            std::thread::spawn(move || {
                let device_info = device_client.get_device_info();

                let c_model = device_info
                    .get_value(DeviceKeys::ProductType, DeviceDomains::All)
                    .unwrap_or_default();
                let c_ios_ver = device_info
                    .get_value(DeviceKeys::ProductVersion, DeviceDomains::All)
                    .unwrap_or_default();

                if device_model != c_model || ios_ver != c_ios_ver {
                    window.emit("carrier_bundle_install_status", false).unwrap();
                    return;
                }

                let install_client = device_client.get_device_installer();
                let install_result = install_client.install_from_path(
                    "/home/Abdullah/iPhone7Plus_iOS_15.8.2_CellularSouthLTE.ipcc",
                    None,
                );

                match install_result {
                    Ok(_) => window.emit("carrier_bundle_install_status", true).unwrap(),
                    Err(_) => window.emit("carrier_bundle_install_status", false).unwrap(),
                };
            });
        }
    }
}

#[tauri::command]
pub fn check_installing_succeed(window: tauri::Window) {
    if let Ok(device_clients) = rsmobiledevice::device::DeviceClient::new() {
        if let Some(device_client) = device_clients.get_first_device() {
            let mut syslog_client = device_client.get_device_syslog();

            // usually there will be a message about the sim being ready in the logs if the carrier
            // bundle installation is good
            syslog_client.set_filter(
                LogFilter::OneShot(Regex::new(r"/\b\w*SIM is Ready\w*\b/i").unwrap()),
                FilterPart::All,
            );

            let window = Arc::new(window);

            let window_clone_1 = Arc::clone(&window);
            let window_clone_2 = Arc::clone(&window);

            // the first callback should be called once the filter succeed to be found and it will
            // stop because we specifed the OneShot, which basically stops the logging if the
            // filter applied
            //
            // if not and it exceeded the timeout, the second callback would get called, thus
            // triggering the false payload
            let _ = syslog_client.log_to_custom_with_timeout_or_else(
                move |_| {
                    window_clone_1
                        .emit("installation_succeed_status", true)
                        .unwrap()
                },
                std::time::Duration::from_secs(40),
                move || {
                    window_clone_2
                        .emit("installation_succeed_status", false)
                        .unwrap()
                },
            );
        }
    }
}

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
