use yew::prelude::*;

use crate::components::ChatComponent;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub id: uuid::Uuid,
}

pub enum RoomMessage {}

pub struct Room {}

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

                <ChatComponent id={ ctx.props().id } />
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }
}
