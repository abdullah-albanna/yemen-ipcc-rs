use base64::{encode, Engine};
use stylist::Style;
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("iphone_screenshot.module.css");

#[function_component]
pub fn IPhoneScreenshot() -> Html {
    // Simulate loading an image file from the web
    let img_data: Vec<u8> =
        include_bytes!("/home/Abdullah/Pictures/pexels-iriser-1379640.jpg").to_vec();

    // Convert the byte array to a base64 string
    let base64_data = encode(&img_data);

    // Create the data URL for the image
    let img_src = format!("data:image/jpeg;base64,{}", base64_data);

    let stylesheet = Style::new(STYLE_FILE).unwrap();
    // Return an HTML structure with the image
    html! {
           <div class={stylesheet}>
               <h1>{ "Image from byte array using the image crate" }</h1>
    //           <img src={img_src} alt="Transformed Image" />
           </div>
       }
}
