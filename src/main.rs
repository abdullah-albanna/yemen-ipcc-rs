mod app;

use app::App;

mod device_management;
mod main_component;
mod options_component;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
