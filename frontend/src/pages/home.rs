use yew::prelude::*;

use crate::components::{RoomCreator, RoomJoiner};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1> { "Welcome to the ITL Chorus" } </h1>
            <RoomJoiner />

            <h2> { "Host a new room" } </h2>
            <RoomCreator />
        </>
    }
}
