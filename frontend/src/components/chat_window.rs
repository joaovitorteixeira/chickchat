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
            // TODO: chat_id should be dynamic
            <MessageList messages=messages chat_id="01JP51CZCJ63QC5F74T9RYDDSA".to_string() />
            <MessageInput />
        </div>
    }
}
