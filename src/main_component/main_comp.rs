use stylist::Style;
use yew::prelude::*;

use super::iphone_information::iphone_info::IPhoneInformation;
use super::iphone_screenshot::iphone_scrnshot::IPhoneScreenshot;

const STYLE: &str = include_str!("./main_comp.module.css");

#[function_component]
pub fn MainComp() -> Html {
    let stylesheet = Style::new(STYLE).unwrap();

    html! {
        <div class={stylesheet} >
            <IPhoneScreenshot />
            <IPhoneInformation />

        </div>
    }
}
