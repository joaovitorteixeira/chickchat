use leptos::{html::Dialog, prelude::*, task::spawn_local};
use leptos_router::hooks::query_signal;
use leptos_use::on_click_outside;

use crate::models::chat::Chat;

#[component]
pub fn NewChatDialog(close_dialog: impl Fn() + 'static + Clone) -> impl IntoView {
    let (chat_name, set_chat_name) = signal("".to_string());
    let close_dialog = close_dialog.clone();
    let target = NodeRef::<Dialog>::new();
    let is_chat_name_empty = move || chat_name.get().is_empty();
    let (_, set_params) = query_signal::<String>("chat_id");
    let create_chat = {
        let close_dialog = close_dialog.clone();
        move |_| {
            let close_dialog = close_dialog.clone();
            let name = chat_name.get();
            let chat: Chat = Chat::new(name.clone());

            spawn_local(async move {
                match chat.create().await {
                    Ok(created_chat) => {
                        set_params.set(created_chat.id.clone());
                        close_dialog();
                    }
                    Err(_e) => {
                        //TODO:  Handle error
                    }
                }
            });
        }
    };

    let _ = on_click_outside(target, move |_| { close_dialog(); });


    view! {
        <dialog class="new-chat-dialog" open node_ref=target>
            <h3>"New Chat 🐥"</h3>
            <input
                type="text"
                placeholder="Chat Name"
                on:input:target=move |ev| {
                    set_chat_name.set(ev.target().value());
                }
                prop:value=chat_name
            />
            <button
                on:click=create_chat
                style="justify-self: center; margin: 1rem;"
                disabled=is_chat_name_empty
            >
                "Create 🐣"
            </button>

        </dialog>
    }
}
