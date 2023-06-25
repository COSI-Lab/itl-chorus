mod chat_input;
mod chat_message;

use std::sync::Arc;

use common::{client_to_server, server_to_client};
use futures::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use yew::prelude::*;

use crate::components::chat::{chat_input::ChatInput, chat_message::ChatMessageComponent};

use self::chat_message::ChatMessageProps;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub id: uuid::Uuid,
}

pub enum Msg {
    Received(server_to_client::Chat),
    SendMsg(client_to_server::Chat),
    Sent,
}

pub struct ChatComponent {
    writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    messages: Vec<(i32, ChatMessageProps)>,
    last_id: i32,
}

impl ChatComponent {
    fn add_message(&mut self, msg: ChatMessageProps) {
        self.last_id += 1;
        self.messages.push((self.last_id, msg));
    }
}

impl Component for ChatComponent {
    type Message = Msg;
    type Properties = ChatProps;

    fn create(ctx: &Context<Self>) -> Self {
        let location = web_sys::window().unwrap().location().host().unwrap();

        let websocket = match WebSocket::open(&format!(
            "ws://{}/api/room/{}/chat",
            location,
            ctx.props().id
        )) {
            Ok(w) => w,
            Err(e) => panic!("{}", e),
        };

        let (writer, read) = websocket.split();

        // Filters ChatMessages.
        let read = read.filter_map(|m| async {
            match m {
                Ok(Message::Text(s)) => {
                    if let Ok(msg) = serde_json::from_str::<server_to_client::Chat>(&s) {
                        Some(Msg::Received(msg))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        });

        ctx.link().send_stream(read);

        Self {
            writer: Arc::new(Mutex::new(writer)),
            messages: vec![],
            last_id: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="boxed">
                    <h2> {"Messages"} </h2>
                    <ul>
                    {
                        for self.messages.iter().map(|(id, msg)| {
                            html! {
                                <li key={*id}>
                                    <ChatMessageComponent name={msg.name.clone()} msg={msg.msg.clone()} />
                                </li>
                            }
                        })
                    }
                    </ul>
                    <h3> {"Send a message"} </h3>
                    <ChatInput callback={ctx.link().callback(|msg| {
                        Msg::SendMsg(client_to_server::Chat {
                            msg
                    })})} />
                </div>
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Received(msg) => {
                self.add_message(ChatMessageProps::from(msg));
                true
            }
            Msg::SendMsg(msg) => {
                let writer = self.writer.clone();

                let message = Message::Text(serde_json::to_string(&msg).unwrap());

                ctx.link().send_future(async move {
                    let mut writer = writer.lock().await;
                    writer.send(message).await.unwrap();
                    Msg::Sent
                });

                false
            }
            // noop
            Msg::Sent => false,
        }
    }
}
