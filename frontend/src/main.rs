mod components;
mod pages;
mod router;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::router::switch;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
