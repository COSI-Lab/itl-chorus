use common::RoomInfo;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    api::{make_request, ApiError},
    router::Route,
};

#[function_component]
pub fn RoomCreator() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        make_request(
            || Request::post("/api/room").build(),
            Callback::from(move |response: Result<RoomInfo, ApiError>| match response {
                Ok(info) => navigator.push(&Route::Room { id: info.id }),
                Err(e) => log::error!("Failed to create room: {}", e),
            }),
        )
    };

    html! {
        <div>
            <button {onclick}> { "Create" } </button>
        </div>
    }
}
