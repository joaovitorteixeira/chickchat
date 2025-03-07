use leptos::prelude::*;

use crate::components::{message_input::MessageInput, message_list::MessageList};

#[component]
pub fn ChatWindow() -> impl IntoView {
    view! {
        <div class="chat-window">
            <MessageList />
            <MessageInput />
        </div>
    }
}
