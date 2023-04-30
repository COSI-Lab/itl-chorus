use yew::prelude::*;

#[function_component]
pub fn RoomCreator() -> Html {
    // When the button is clicked, we send a post request to the server and navigate to the created room.
    // The server will respond with the room code.

    html! {
        <div>
            <button> { "Create" } </button>
        </div>
    }
}
