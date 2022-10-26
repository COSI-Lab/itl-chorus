use yew::prelude::*;

pub mod components;

use components::UploadMidi;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1> { "Welcome to the ITL Chorus" } </h1>

            <h2> { "Room Code" } </h2>
            <button> { "Join" } </button>

            <h2> { "Host a new room" } </h2>
            <button> { "Create" } </button>

            <h2> { "most recently used" } </h2>
            <h2> { "most frequently used" } </h2>

            <UploadMidi></UploadMidi>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
