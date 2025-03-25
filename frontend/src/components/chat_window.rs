use leptos::prelude::*;

use crate::{
    components::{message_input::MessageInput, message_list::MessageList},
    models::{chat::Chat, message::MessageWithStatus},
};

#[component]
pub fn ChatWindow() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<MessageWithStatus>::new());
    let chat_id = Chat::get_id_from_query();
    provide_context(set_messages);

    view! {
        <div class="chat-window">
            {move || {
                let chat_id = chat_id.get();
                set_messages.write().clear();
                if chat_id.is_none() {
                    return view! {
                        <div class="chat-window__empty">
                            {"Select a chat to start chatting"} <br /> {"🐤"}
                        </div>
                    }
                        .into_any();
                } else {
                    view! { <MessageList messages=messages chat_id=chat_id.unwrap() /> }.into_any()
                }
            }} <MessageInput />
        </div>
    }
}
