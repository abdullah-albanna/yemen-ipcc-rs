use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::device_management::device_context::DeviceProvider;
use crate::main_component::MainComp;
use crate::options_component::OptionsComp;

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
