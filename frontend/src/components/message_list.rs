use std::{rc::Rc, sync::Arc};

use leptos::{html::Div, prelude::*, task::spawn_local,};
use leptos_use::{core::Direction, use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use crate::models::{message::{Message, MessageWithStatus}, user::User};

type HasMoreTuple = (ReadSignal<bool>,  WriteSignal<bool>); 
type MessagesTuple = (ReadSignal<Vec<MessageWithStatus>>,WriteSignal<Vec<MessageWithStatus>> );

async fn get_old_messages(
    (has_more, set_has_more): HasMoreTuple, 
    (messages, set_messages): MessagesTuple, 
    chat_id: String
) {
    if has_more.get_untracked() == false {
        return;
    }
    let get_last_message_id = move || messages.read_untracked().last().map(|m| m.message.id.clone()).flatten();
    let old_messages = Message::list(10, get_last_message_id(), chat_id.clone()).await;
    
    set_has_more.set(!old_messages.is_empty());
    set_messages.update(|data| {
        data.extend(old_messages);
    });
}

fn load_messages_on_mount(
    (has_more, set_has_more): HasMoreTuple, 
    (messages, set_messages): MessagesTuple, 
    div_element: NodeRef<Div>,
    chat_id: String
) -> String {
    let cloned_chat_id = Rc::new(chat_id.clone());

    Effect::new(move || {
        let chat_id = Rc::clone(&cloned_chat_id);
        spawn_local(async move {
            while has_more.get_untracked() {
                get_old_messages((has_more, set_has_more), (messages, set_messages), chat_id.to_string()).await;
                let div_el = div_element.get_untracked().unwrap();

                if div_el.scroll_height() > div_el.client_height() {
                    break;
                }
            }
        });
    });

    chat_id.to_string()
}

#[component]
pub fn MessageList(messages: ReadSignal<Vec<MessageWithStatus>>, chat_id: String) -> impl IntoView {
    let set_messages = use_context::<WriteSignal<Vec<MessageWithStatus>>>().expect("to have found the setter provided");
    let (has_more, set_has_more) = signal(true);
    let el = NodeRef::<Div>::new();
    let user = User::get_user_from_session_storage();
    let chat_id = load_messages_on_mount(
        (has_more, set_has_more), 
        (messages, set_messages), 
        el, 
        chat_id
    );
    //TODO: any way to improve this?
    let chat_id = Arc::new(chat_id);

    Effect::new({
        let chat_id = Arc::clone(&chat_id);
        move |_| {
            Message::on_message(chat_id.to_string(), set_messages);
        }
    });

    let _= use_infinite_scroll_with_options(
        el,
        move |_| {
            let chat_id = Arc::clone(&chat_id);
            async move {
                get_old_messages((has_more, set_has_more), (messages, set_messages),chat_id.to_string()).await;       
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
                    .map(|(_, new_message)| {
                        view! {
                            <div
                                class="message-item"
                                style:align-self=if new_message.message.user_id != user.id {
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
