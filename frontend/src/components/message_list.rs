use std::sync::Arc;

use leptos::{html::Div, prelude::*, task::spawn_local};
use leptos_use::{core::Direction, use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use crate::models::message::{Message, MessageWithStatus};

#[component]
pub fn MessageList(messages: ReadSignal<Vec<MessageWithStatus>>, chat_id: String) -> impl IntoView {
    let set_message = use_context::<WriteSignal<Vec<MessageWithStatus>>>().expect("to have found the setter provided");
    let (has_more, set_has_more) = signal(true);
    let el = NodeRef::<Div>::new();
    //TODO: any way to improve this?
    let chat_id = Arc::new(chat_id);
    let get_old_messages= async move |chat_id: String| -> bool {
        if has_more.get() == false {
            return false;
        }

        let old_messages = Message::list(10, messages.read().last().map(|m| m.message.id.clone()).flatten(), chat_id).await;
        
        set_has_more.set(!old_messages.is_empty());
        set_message.update(|data| {
            data.extend(old_messages);
        });

        has_more.get()
    };

    Effect::new({
        let chat_id = Arc::clone(&chat_id);
        move |_| {
            let chat_id = Arc::clone(&chat_id);
            spawn_local(async move {
                while has_more.get(){
                    get_old_messages(chat_id.to_string()).await;
                    let div_el = el.get().unwrap();

                    if div_el.scroll_height() > div_el.client_height() {
                        break;
                    }
                }
            });
        }
    });

    let _= use_infinite_scroll_with_options(
        el,
        move |_| {
            let chat_id = Arc::clone(&chat_id);
            async move {
                get_old_messages(chat_id.to_string()).await;       
        }},
        UseInfiniteScrollOptions::default().distance(10.0).direction(Direction::Top),
    );

    view! {
        <div class="message-list" node_ref=el>
            {move || {
                messages
                    .read()
                    .iter()
                    .enumerate()
                    .map(|(index, new_message)| {
                        view! {
                            <div
                                class="message-item"
                                style:align-self=if index % 2 == 0 {
                                    "flex-start"
                                } else {
                                    "flex-end"
                                }
                            >
                                {new_message.message.content.clone()}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
