use leptos::prelude::*;

use crate::{
    components::{message_input::MessageInput, message_list::MessageList},
    models::message::MessageWithStatus,
};

#[component]
pub fn ChatWindow() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<MessageWithStatus>::new());

    provide_context(set_messages);

    view! {
        <div class="chat-window">
            <MessageList messages=messages chat_id="TODO".to_string() />
            <MessageInput />
        </div>
    }
}
