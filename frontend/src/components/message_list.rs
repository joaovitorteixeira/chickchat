use leptos::prelude::*;

use crate::models::message::MessageWithStatus;

#[component]
pub fn MessageList(messages: ReadSignal<Vec<MessageWithStatus>>) -> impl IntoView {
    view! {
        <div class="message-list">
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
