use std::{pin::Pin, rc::Rc, sync::Arc};
use leptos::{html::Div, leptos_dom::logging, prelude::*, task::spawn_local};
use leptos_use::{core::Direction, use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use crate::{models::{ message::{Message, MessageWithStatus}, user::User}, util::infinite_scroll::InfiniteScroll};
use send_wrapper::SendWrapper;


fn get_message_fetcher(chat_id: String) -> impl Fn(u8, Option<String>) -> Pin<Box<dyn std::future::Future<Output = Vec<MessageWithStatus>>>> {
    move |limit, last_id| {
        Box::pin(Message::list(limit, last_id, chat_id.clone()))
    }
}

fn load_messages_on_mount(
    infinite_scroll: Arc<InfiniteScroll<MessageWithStatus>>,
    div_element: NodeRef<Div>,
    chat_id: String
) -> String {
    let cloned_chat_id = Rc::new(chat_id.clone());

    Effect::new(move || {
        let chat_id = Rc::clone(&cloned_chat_id);
        let infinite_scroll = Arc::clone(&infinite_scroll);

        spawn_local(async move {
            let (has_more, _) = infinite_scroll.has_more_tuple;
            let (messages, _) = infinite_scroll.messages_tuple;
            let get_last_id = || messages.read_untracked().last().and_then(|m| m.message.id.clone());
            while has_more.get_untracked() {
                infinite_scroll.get_old_items(get_last_id, get_message_fetcher(chat_id.to_string())).await;
                let div_el = div_element.get_untracked().unwrap();

                if div_el.scroll_height() > div_el.client_height() {
                    break;
                }
            }
        })
    });

    chat_id.to_string()
}

#[component]
pub fn MessageList(messages: ReadSignal<Vec<MessageWithStatus>>, chat_id: String) -> impl IntoView {
    let set_messages = use_context::<WriteSignal<Vec<MessageWithStatus>>>().expect("to have found the setter provided");
    let (has_more, set_has_more) = signal(true);
    let el = NodeRef::<Div>::new();
    let user = User::get_user_from_session_storage();
    let infinity_scroll = Arc::new(InfiniteScroll::new(
        (has_more, set_has_more),
        (messages, set_messages),
    ));
    let chat_id = Arc::new(load_messages_on_mount(
        Arc::clone(&infinity_scroll),
        el, 
        chat_id.clone()
    ));
    let get_last_message_id = move || messages.read().last().and_then(|m| m.message.id.clone());
    let event_source = SendWrapper::new(Message::on_message(chat_id.to_string(), set_messages.clone()));

    on_cleanup(move || {
        logging::console_log("Closing event source");
        event_source.close();
    });

    let _= use_infinite_scroll_with_options(
        el,
        move |_| {
            let chat_id = Arc::clone(&chat_id);
            let infinity_scroll = Arc::clone(&infinity_scroll);
            async move {
                infinity_scroll.get_old_items(
                    || get_last_message_id(),
                    get_message_fetcher(chat_id.to_string()),
                ).await;
            }
        },
        UseInfiniteScrollOptions::default().distance(10.0).direction(Direction::Top),
    );

    view! {
        <div class="message-list" node_ref=el>
            {move || {
                messages
                    .read()
                    .iter()
                    .map(|message| {
                        view! {
                            <div
                                class="message-item"
                                style:align-self=if message.message.member_id != user.id {
                                    "flex-start"
                                } else {
                                    "flex-end"
                                }
                            >
                                {message.message.content.clone()}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
