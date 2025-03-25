use crate::components::{chat_list::ChatList, new_chat_dialog::NewChatDialog};
use leptos::prelude::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (is_dialog_open, set_dialog_open) = signal(false);
    let open_dialog = move |_| set_dialog_open.set(true);
    let close_dialog = move || set_dialog_open.set(false);

    view! {
        <div class="chat-sidebar">
            <h3>"Chickchat 🐣"</h3>
            <button class="new-chat-button" on:click=open_dialog>
                "New Chat 🐥"
            </button>
            <ChatList />

            {move || {
                if is_dialog_open.get() {
                    view! { <NewChatDialog close_dialog=close_dialog /> }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </div>
    }
}
