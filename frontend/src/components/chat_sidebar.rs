use leptos::prelude::*;

use crate::components::chat_list::ChatList;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="chat-sidebar">
            <h3>"Chickchat 🐣"</h3>
            <ChatList />
        </div>
    }
}
