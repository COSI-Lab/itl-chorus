use std::sync::Arc;

use common::{
    client_to_server::{ClientToServerWS, UploadMessage},
    server_to_client::ServerToClientWS,
};
use futures::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use yew::prelude::*;

use crate::components::{chat_input::ChatInput, chat_message::ChatMessage};

use super::chat_message::ChatMessageProps;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub id: uuid::Uuid,
}

pub enum Msg {
    Received(ServerToClientWS),
    SendMsg(String),
    Sent,
}

pub struct Chat {
    writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    messages: Vec<(i32, ChatMessageProps)>,
    last_id: i32,
}

impl Chat {
    fn add_message(&mut self, msg: ChatMessageProps) {
        self.last_id += 1;
        self.messages.push((self.last_id, msg));
    }
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ChatProps;

    fn create(ctx: &Context<Self>) -> Self {
        let location = web_sys::window().unwrap().location().host().unwrap();

        let websocket =
            match WebSocket::open(&format!("ws://{}/api/room/{}/ws", location, ctx.props().id)) {
                Ok(w) => w,
                Err(e) => panic!("{}", e),
            };

        let (writer, read) = websocket.split();

        // Filters ChatMessages.
        let read = read.filter_map(|m| async {
            match m {
                Ok(Message::Text(s)) => {
                    if let Ok(msg) = serde_json::from_str::<ServerToClientWS>(&s) {
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
                <h2> {"Messages"} </h2>
                <ul>
                {
                    for self.messages.iter().map(|(id, msg)| {
                        html! {
                            <li key={*id}>
                                <ChatMessage name={msg.name.clone()} msg={msg.msg.clone()} />
                            </li>
                        }
                    })
                }
                </ul>
                <h3> {"Send a message"} </h3>
                <ChatInput callback={ctx.link().callback(|s| {
                    Msg::SendMsg(s)
                })} />
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Received(msg) => match msg {
                ServerToClientWS::Chat(msg) => {
                    self.add_message(ChatMessageProps {
                        name: msg.name.to_string(),
                        msg: msg.msg,
                    });
                    true
                }
                ServerToClientWS::Join(msg) => {
                    self.add_message(ChatMessageProps {
                        name: "Server".to_string(),
                        msg: format!("{} joined the room", msg.name),
                    });
                    true
                }
                ServerToClientWS::Leave(msg) => {
                    self.add_message(ChatMessageProps {
                        name: "Server".to_string(),
                        msg: format!("{} left the room", msg.name),
                    });
                    true
                }
            },
            Msg::SendMsg(msg) => {
                let writer = self.writer.clone();
                let message = ClientToServerWS::Upload(UploadMessage { msg });
                let message = Message::Text(serde_json::to_string(&message).unwrap());

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
