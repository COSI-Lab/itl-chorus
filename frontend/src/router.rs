use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/room/:id")]
    Room { id: uuid::Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! {
                <pages::Home />
            }
        }
        Route::Room { id } => {
            html! {
                <pages::Room id={id} />
            }
        }
        Route::NotFound => {
            html! {
                <p> { "404" } </p>
            }
        }
    }
}
