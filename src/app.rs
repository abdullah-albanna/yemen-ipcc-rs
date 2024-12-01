use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::device_management::device_context::DeviceProvider;
use crate::main_component::MainComp;
use crate::options_component::OptionsComp;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component]
pub fn App() -> Html {
    html! {
        <DeviceProvider >
            <div class="container">
                <MainComp />
                <OptionsComp / >
            </div>
        </DeviceProvider>
    }
}
