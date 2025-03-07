use leptos::prelude::*;

use crate::components::message_input::MessageInput;

#[component]
pub fn ChatWindow() -> impl IntoView {
    view! {
        <div class="chat-window">
            <MessageInput />
        </div>
    }
}
