use stylist::{yew::styled_component, Style};
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement};
use yew::prelude::*;

use super::info_boxes::{BatteryInfo, DeviceInfo, OSInfo, StorageInfo};

const STYLE_FILE: &str = include_str!("iphone_info.module.css");

fn copy_to_clipboard(e: MouseEvent) {
    if let Some(div_element) = e.target_dyn_into::<HtmlElement>() {
        let mut text = String::new();

        if let Some(parent) = div_element.offset_parent() {
            let html_element = parent.unchecked_into::<HtmlElement>();
            text = html_element.inner_text();
        }

        if let Some(window) = window() {
            let clipboard = window.navigator().clipboard();
            let _ = clipboard.write_text(&text);
        }
    }
}

#[function_component]
pub fn IPhoneInformation() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    let handle_info_click = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        copy_to_clipboard(e);
    });

    html! {
            <div class={stylesheet}>
                <DeviceInfo  onclick={&handle_info_click} />
                <OSInfo      onclick={&handle_info_click} />
                <BatteryInfo onclick={&handle_info_click} />
                <StorageInfo onclick={&handle_info_click} />
            </div>
    }
}
