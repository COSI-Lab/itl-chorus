use yew::prelude::*;

use crate::components::Chat;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub id: uuid::Uuid,
}

pub enum RoomMessage {}

pub struct Room {}

impl Component for Room {
    type Message = RoomMessage;
    type Properties = RoomProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ format!("Room {}", ctx.props().id) }</p>

                <Chat id={ ctx.props().id } />
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }
}
