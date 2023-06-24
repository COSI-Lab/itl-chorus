use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

use crate::components::Chat;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub id: uuid::Uuid,
}

pub enum RoomMessage {}

pub struct Room {}

#[wasm_bindgen]
extern "C" {
    fn play();
}

impl Component for Room {
    type Message = RoomMessage;
    type Properties = RoomProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ format!("Room {}", ctx.props().id) }</p>

                <Chat id={ ctx.props().id } />

                <button onclick={ Callback::from(|_| play()) }> { "Play" } </button>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
