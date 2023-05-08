use std::sync::Arc;

use futures::{lock::Mutex, stream::SplitSink, SinkExt, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message};
use yew::prelude::*;

use crate::components::chat_input::ChatInput;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub id: uuid::Uuid,
}

pub enum ChatMessage {
    Received(common::Message),
    Send(String),
    Sent,
}

pub struct Chat {
    writer: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    messages: Vec<(i32, common::Message)>,
    last_id: i32,
}

impl Component for Chat {
    type Message = ChatMessage;
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
                    if let Ok(msg) = serde_json::from_str::<common::Message>(&s) {
                        Some(ChatMessage::Received(msg))
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
                    self.messages.iter().map(|msg|
                        html!{<li key={msg.0.to_string()}> {format!("{}: {}", msg.1.name, msg.1.msg)} </li>}
                    ).collect::<Html>()
                }
                </ul>
                <h3> {"Send a message"} </h3>
                <ChatInput callback={ctx.link().callback(|s| {
                    ChatMessage::Send(s)
                })} />
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChatMessage::Received(msg) => {
                self.messages.push((self.last_id, msg));
                self.last_id += 1;
                true
            }
            ChatMessage::Send(msg) => {
                let writer = self.writer.clone();

                ctx.link().send_future(async move {
                    let mut writer = writer.lock().await;

                    let message = serde_json::to_string(&common::UploadMessage { msg }).unwrap();
                    writer.send(Message::Text(message)).await.unwrap();

                    ChatMessage::Sent
                });

                false
            }
            ChatMessage::Sent => false,
        }
    }
}
