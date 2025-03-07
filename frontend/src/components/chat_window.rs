use leptos::prelude::*;

use crate::components::{message_input::MessageInput, message_list::MessageList};

#[component]
pub fn ChatWindow() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<String>::new());
    view! {
        <div class="chat-window">
            <MessageList messages=messages />
            <MessageInput set_message=set_messages />
        </div>
    }
}
