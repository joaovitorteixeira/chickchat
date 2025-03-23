use std::sync::Arc;

use crate::{models::chat::Chat, util::infinite_scroll::InfiniteScroll};
use leptos::{html::Div, prelude::*, reactive::wrappers::write::SignalSetter, Params};
use leptos_router::{hooks::query_signal, params::Params};
use leptos_use::use_infinite_scroll;

#[derive(Params, PartialEq)]
pub struct ChatQueryParam {
    chat_id: Option<String>,
}

#[component]
pub fn ChatList() -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let (has_more, set_has_more) = signal(true);
    let (chats, set_chats) = signal(Vec::<Chat>::new());
    let infinite_scroll = Arc::new(InfiniteScroll::new((has_more, set_has_more), (chats, set_chats)));
    let get_last_chat_id = move || chats.read().last().and_then(|m| m.id.clone());
    let (_, set_params) = query_signal::<String>("chat_id");
    let _ = use_infinite_scroll(
        el,
        move |_| {
            let infinite_scroll = Arc::clone(&infinite_scroll);
            
            async move {
                infinite_scroll.get_old_items(get_last_chat_id, Chat::list).await;
            }
        } 
    );
    
    view! {
        <div class="chat-list" node_ref=el>

            {move || {
                chats
                    .read()
                    .iter()
                    .map(|chat| {
                        let (on_click, chat) = (move |set_params: SignalSetter<Option<String>>| {
                            let chat_id = chat.id.clone();
                            (
                                move |_| {
                                    set_params.set(chat_id.clone());
                                },
                                chat,
                            )
                        })(set_params);
                        view! {
                            <button class="chat-item" on:click=on_click>
                                {chat.name.clone()}
                            </button>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
