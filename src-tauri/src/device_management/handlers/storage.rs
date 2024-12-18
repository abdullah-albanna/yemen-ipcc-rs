use serde::Serialize;

use rsmobiledevice::{
    device::DeviceClient, device_info::domains::DeviceDomains, devices_collection::SingleDevice,
};

#[derive(Serialize, Clone)]
pub struct Storage {
    pub total_storage: u64,
    pub used_storage: u64,
    pub available_storage: u64,
}

// Function to handle a connected device

pub fn handle_device_storage(device: &DeviceClient<SingleDevice>) -> Storage {
    let device_info = device.get_device_info();

    let disk_dict = device_info.get_values(DeviceDomains::DiskUsage).unwrap();
    let total_storage = disk_dict
        .get("totaldiskcapacity")
        .unwrap()
        .to_owned()
        .parse::<u64>()
        .unwrap();
    let available_storage = disk_dict
        .get("amountrestoreavailable")
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
