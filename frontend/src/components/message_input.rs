use leptos::prelude::*;

#[component]
pub fn MessageInput() -> impl IntoView {
    view! {
        <div class="message-input-container">
            <textarea class="message-input" type="text" placeholder="Type your message..." />
            <button class="message-send">"Send"</button>
        </div>
    }
}
