use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use super::inject_button::InjectButton;

const STYLE_FILE: &str = include_str!("opt_comp.module.css");

#[styled_component]
pub fn OptionsComp() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={stylesheet}>
            <h3 class="options-label">{ "Options" }</h3>

            <label for="bundles">{ "Select a bundle" }</label>
            <select id="bundles" >
                <option value="CellularSouthLTE">{ "CellularSouthLTE" }</option>
                <option value="CWW">{ "CWW" }</option>
                <option value="USCellularLTE">{ "USCellularLTE" }</option>
                <option value="China">{ "China" }</option>
            </select>

            <label for="container">{ "Select a container" }</label>
            <select id="container" >
                <option value="unknown.bundle">{ "unknown.bundle" }</option>
                <option value="default.bundle">{ "default.bundle" }</option>
            </select>
            <InjectButton />
        </div>
    }
}
