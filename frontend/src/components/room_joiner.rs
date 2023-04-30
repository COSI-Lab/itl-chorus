// This component is responsible for joining a room.
//
// The user enters the room code and clicks join, the callback will be given the room code.

use yew::prelude::*;

#[function_component]
pub fn RoomJoiner() -> Html {
    html! {
        <div>
            <button> { "Join" } </button>
        </div>
    }
}
