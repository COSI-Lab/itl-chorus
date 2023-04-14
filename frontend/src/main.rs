use frontend::components::{UploadMidi, RoomJoiner};

use yew::{function_component, Html, html};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1> { "Welcome to the ITL Chorus" } </h1>

            <RoomJoiner></RoomJoiner>

            <h2> { "Host a new room" } </h2>
            <button> { "Create" } </button>

            <h2> { "most recently used" } </h2>
            <h2> { "most frequently used" } </h2>

            <UploadMidi></UploadMidi>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}