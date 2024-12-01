use crate::device_management::{
    device::DeviceBattery,
    device_context::{DeviceContext, UpdateOptions},
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Prop {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn BatteryInfo(prop: &Prop) -> Html {
    let context = use_context::<DeviceContext>().expect("Device context not found");

    use_memo(context.device_info.status, |_| {
        context
            .set_device_info
            .emit(UpdateOptions::Battery(DeviceBattery::new(20, 80, 240)))
    });

    html! {
        <fieldset class="grid-item">
            <legend>{"Battery"}</legend>
            <div class="row3" onclick={&prop.onclick}>
                <h4>{format!("Battery Level: {}", context.device_info.battery.get_battery_level())}</h4>
                <h4>{format!("Battery Health {}", context.device_info.battery.get_battery_health())}</h4>
                <h4>{format!("Cycle Count: {}", context.device_info.battery.get_cycle_counts())}</h4>
            </div>
        </fieldset>
    }
}
