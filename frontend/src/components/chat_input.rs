use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::util::get_value_from_input_event;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub callback: Callback<String>,
}

/// A chat input component. This component is used to send messages to the chat.
#[function_component(ChatInput)]
pub fn chat_input(props: &InputProps) -> Html {
    let input_node_ref = use_node_ref();
    let msg: UseStateHandle<Option<String>> = use_state(|| None);

    let on_msg_change = {
        let msg: UseStateHandle<Option<String>> = msg.clone();

        Callback::from(move |event: InputEvent| {
            let value = get_value_from_input_event(event);
            msg.set(Some(value));
        })
    };

    let input = html!(
        <input ref={input_node_ref.clone()} type="text" placeholder="Message" oninput={on_msg_change} />
    );

    let on_send = {
        let msg = msg;
        let cb = props.callback.clone();

        Callback::from(move |_| {
            if let Some(msg) = msg.as_ref() {
                cb.emit(msg.clone());
                if let Some(info) = input_node_ref.cast::<HtmlInputElement>() {
                    info.set_value("");
                }
            }
        })
    };

    html! {
        <div>
            { input }
            <button onclick={on_send}> { "Send" } </button>
        </div>
    }
}
