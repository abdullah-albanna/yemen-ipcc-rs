use crate::device_management::{
    device::DeviceStorage,
    device_context::{DeviceContext, UpdateOptions},
};
use gloo::timers::callback::Interval;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Prop {
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn StorageInfo(prop: &Prop) -> Html {
    let context = use_context::<DeviceContext>().expect("Device context not found");

    use_memo(context.device_info.status, |_| {
        context
            .set_device_info
            .emit(UpdateOptions::Storage(DeviceStorage::new(256, 39)))
    });

    html! {
        <fieldset class="grid-item">
            <legend>{"Storage"}</legend>
            <div class="row4" onclick={&prop.onclick}>
                <h4>{format!("Total Storage: {}", context.device_info.storage.get_total_storage())}</h4>
                <h4>{format!("Used Storage: {}", context.device_info.storage.get_used_storage())}</h4>
                <h4>{format!("Available Storage: {}", context.device_info.storage.get_availabel_storage())}</h4>
            </div>
        </fieldset>
    }
}
