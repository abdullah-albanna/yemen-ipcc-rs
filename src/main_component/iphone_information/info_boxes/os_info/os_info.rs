use crate::device_management::{
    device::{DeviceBattery, DeviceModel, OS},
    device_context::{DeviceContext, UpdateOptions},
};
use yew::prelude::*;

use gloo::timers::callback::Interval;
#[derive(Properties, PartialEq, Clone)]
pub struct Prop {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn OSInfo(prop: &Prop) -> Html {
    let context = use_context::<DeviceContext>().expect("Device context not found");

    use_memo(context.device_info.status, |_| {
        context
            .set_device_info
            .emit(UpdateOptions::OS(OS::new("18.1", "12Jsdc")))
    });

    html! {
        <fieldset class="grid-item">
            <legend>{"OS"}</legend>
            <div class="row2" onclick={&prop.onclick}>
                <h4>{format!("iOS: {}", context.device_info.os.get_ios_ver())}</h4>
                <h4>{format!("Build Number: {}", context.device_info.os.get_build_num())}</h4>
            </div>
        </fieldset>
    }
}
