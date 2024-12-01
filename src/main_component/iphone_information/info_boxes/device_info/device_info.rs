use crate::device_management::{
    device::{DeviceModel, ProductType},
    device_context::{DeviceContext, UpdateOptions},
};
use gloo::console::log;
use gloo::timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Prop {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn DeviceInfo(prop: &Prop) -> Html {
    let context = use_context::<DeviceContext>().expect("Device context not found");

    use_memo(context.device_info.status, |_| {
        context
            .set_device_info
            .emit(UpdateOptions::Device(DeviceModel::new(
                ProductType::iPhone13Pro,
                "133jn",
            )))
    });

    html! {
        <fieldset class="grid-item">
            <legend>{"Device Information"}</legend>
            <div class="row1" onclick={&prop.onclick}>
                <h4>{format!("Model: {}", context.device_info.device.get_model())}</h4>
                <h4>{format!("Serial Number: {}", context.device_info.device.get_serial())}</h4>
            </div>
        </fieldset>
    }
}
