// This component is responsible for joining a room.
//
// The user enters the room code and clicks join, the callback will be given the room code.
//
// For now the room code is a uuid pasted as text

use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{router::Route, util::get_value_from_input_event};

#[function_component]
pub fn RoomJoiner() -> Html {
    let navigator = use_navigator().unwrap();
    let room_code: UseStateHandle<Option<String>> = use_state(|| None);

    let on_join = {
        let room_code = room_code.clone();
        Callback::from(move |_| {
            let room_code = room_code.as_ref().unwrap();
            let uuid = uuid::Uuid::parse_str(room_code).unwrap();
            navigator.push(&Route::Room { id: uuid });
        })
    };

    let on_room_code_change = Callback::from(move |event: InputEvent| {
        let value = get_value_from_input_event(event);
        room_code.set(Some(value));
    });

    html! {
        <div>
            <input  type="text" placeholder="Room Code" oninput={on_room_code_change} />
            <button onclick={on_join}> { "Join" } </button>
        </div>
    }
}
