use leptos::prelude::*;

#[component]
pub fn ChatList() -> impl IntoView {
    let number_of_chats = 100;

    view! {
        <div class="chat-list">
            {
                let chat_items: Vec<_> = (0..number_of_chats)
                    .map(|i| {
                        view! {
                            <div class="chat-item">
                                <div class="chat-item-title">{format!("Chat {}", i)}</div>
                            </div>
                        }
                    })
                    .collect();
                view! { <>{chat_items}</> }
            }
        </div>
    }
}
