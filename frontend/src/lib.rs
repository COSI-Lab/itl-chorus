mod api;
mod components;
mod midiplayer;
mod pages;
mod router;
mod util;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

use crate::router::switch;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<router::Route> render={switch} />
        </BrowserRouter>
    }
}
