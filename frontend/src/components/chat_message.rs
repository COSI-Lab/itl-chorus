use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatMessageProps {
    pub name: String,
    pub msg: String,
}

/// A chat message
#[function_component(ChatMessage)]
pub fn chat_message(props: &ChatMessageProps) -> Html {
    html! {
        <div>
            <span>{ &props.name }</span>
            <span>{ &props.msg }</span>
        </div>
    }
}
