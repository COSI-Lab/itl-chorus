use uuid::Uuid;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::router::Route;

#[function_component]
pub fn RoomCreator() -> Html {
    let location = web_sys::window().unwrap().location();
    let navigator = use_navigator().unwrap();

    // When the button is clicked, we send a post request to the server and navigate to the created room.
    let onclick = {
        Callback::from(move |_| {
            let navigator = navigator.clone();
        
            let url = format!("{}/api/room", location.origin().unwrap());
            log::debug!("Creating room at {}", url);

            let request = async {
                let response = reqwest::Client::new().post(url).send().await?;
                log::debug!("Response: {:?}", response);
                let room = response.text().await?;

                Ok::<Uuid, anyhow::Error>(Uuid::parse_str(&room)?)
            };

            wasm_bindgen_futures::spawn_local(async move {
                match request.await {
                    Ok(room) => navigator.replace(&Route::Room { id: room }),
                    Err(e) => log::error!("Failed to create room: {}", e),
                }
            });
        })
    };

    html! {
        <div>
            <button {onclick}> { "Create" } </button>
        </div>
    }
}
