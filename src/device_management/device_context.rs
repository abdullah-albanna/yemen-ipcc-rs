use crate::device_management::device::{DeviceInfo, DeviceModel, DeviceStatus, ProductType, OS};
use gloo::timers::callback::Interval;
use yew::{
    function_component, html, use_effect_with, use_state, Children, ContextProvider, Html,
    Properties,
};

use super::device::{DeviceBattery, DeviceStorage};

#[derive(Clone, PartialEq, Debug)]
pub struct DeviceContext {
    pub device_info: DeviceInfo,
    pub set_device_info: yew::Callback<UpdateOptions>,
}

#[derive(Properties, PartialEq)]
pub struct DeviceProviderProps {
    pub children: Children,
}

// UpdateOptions allows partial updates of DeviceInfo fields
#[derive(Clone, Debug)]
pub enum UpdateOptions {
    Battery(DeviceBattery),
    Storage(DeviceStorage),
    Status(DeviceStatus),
    OS(OS),
    Device(DeviceModel),
}

impl DeviceInfo {
    // Apply the update options to update specific fields of DeviceInfo
    pub fn apply_update(&mut self, options: UpdateOptions) {
        match options {
            UpdateOptions::Device(device) => self.device = device,
            UpdateOptions::Status(status) => self.status = status,
            UpdateOptions::OS(os) => self.os = os,
            UpdateOptions::Storage(storage) => self.storage = storage,
            UpdateOptions::Battery(battery) => self.battery = battery,
        }
    }
}

#[function_component(DeviceProvider)]
pub fn device_provider(props: &DeviceProviderProps) -> Html {
    let device_info = use_state(DeviceInfo::default);

    // The callback now accepts UpdateOptions to allow partial updates
    let set_device_info = {
        let device_info = device_info.clone();
        yew::Callback::from(move |options: UpdateOptions| {
            device_info.set({
                let mut updated_info = (*device_info).clone();
                updated_info.apply_update(options);
                updated_info
            });
        })
    };

    // Periodic check effect
    {
        let set_device_info = set_device_info.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(1_000, move || {
                // Simulate checking device status (replace with real logic)
                let status = check_device_status(); // Replace with your function

                set_device_info.emit(UpdateOptions::Status(status));
                // Example update: only update the battery and status
            });
            || drop(interval) // Clean-up function to stop the interval
        });
    }

    html! {
        <ContextProvider<DeviceContext> context={DeviceContext {
            device_info: (*device_info).clone(),
            set_device_info,
        }}>
            { for props.children.iter() }
        </ContextProvider<DeviceContext>>
    }
}
// Mock function to check device status (replace this with actual logic)
fn check_device_status() -> DeviceStatus {
    // Simulate checking connected status
    // Replace this with code that interacts with your backend/Tauri API
    let is_connected = rand::random::<bool>(); // Example: random true/false
    if is_connected {
        DeviceStatus::Connected
    } else {
        DeviceStatus::Disconnected
    }
}
