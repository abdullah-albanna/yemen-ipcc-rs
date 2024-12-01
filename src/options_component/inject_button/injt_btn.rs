use stylist::{yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("inject_button.module.css");

#[styled_component]
pub fn InjectButton() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
           <button class={stylesheet}>{ "inject" }</button>
    }
}
