use common::server_to_client;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatMessageProps {
    pub name: AttrValue,
    pub msg: AttrValue,
}

impl From<server_to_client::Chat> for ChatMessageProps {
    fn from(value: server_to_client::Chat) -> Self {
        use server_to_client::Chat::*;

        match value {
            Message { name, msg } => Self {
                name: name.to_string().into(),
                msg: msg.into(),
            },
            Joined { name } => {
                let msg = format!("{} joined", name);
                Self {
                    name: "Server".into(),
                    msg: msg.into(),
                }
            }
            Left { name } => {
                let msg = format!("{} left", name);
                Self {
                    name: "Server".into(),
                    msg: msg.into(),
                }
            }
            HostAction { msg } => Self {
                name: "Server".into(),
                msg: msg.into(),
            },
        }
    }
}

/// A chat message
#[function_component(ChatMessageComponent)]
pub fn chat_message(props: &ChatMessageProps) -> Html {
    html! {
        <div>
            <span>{ &props.name }</span>
            <span>{ ":" }</span>
            <span>{ &props.msg }</span>
        </div>
    }
}
