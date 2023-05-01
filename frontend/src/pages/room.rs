use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub id: uuid::Uuid,
}

pub struct Room {}

impl Component for Room {
    type Message = ();
    type Properties = RoomProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Room {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p>{ format!("Room {}", ctx.props().id) }</p>
        }
    }
}
