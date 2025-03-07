use leptos::prelude::*;

#[component]
pub fn MessageList(messages: ReadSignal<Vec<String>>) -> impl IntoView {
    view! {
        <div class="message-list">
            {move || {
                messages
                    .read()
                    .iter()
                    .enumerate()
                    .map(|(index, message)| {
                        view! {
                            <div
                                class="message-item"
                                style:align-self=if index % 2 == 0 {
                                    "flex-start"
                                } else {
                                    "flex-end"
                                }
                            >
                                {message.clone()}
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
