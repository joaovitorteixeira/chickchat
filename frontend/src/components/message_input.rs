use leptos::{
    ev::Targeted, prelude::*, web_sys::{Event, HtmlTextAreaElement, KeyboardEvent}
};
use leptos_icons::Icon;
use icondata as icon;
use crate::models::{message::{Message, MessageWithStatus}, user::User};

async fn send_message(input: &str) -> MessageWithStatus {
        let user = User::get_user_from_session_storage();
        let mut message = Message::new(input.to_string(), user.id, "TODO".to_string());
        let message_data = message.send().await;

        message_data

}

#[component]
pub fn MessageInput() -> impl IntoView {
    let set_message = use_context::<WriteSignal<Vec<MessageWithStatus>>>().expect("to have found the setter provided");
    let (input, set_input) = signal(String::new());
    let (is_sending, set_is_sending) = signal(false);
    let _ = LocalResource::new(move || {
        let is_sending = is_sending.get();
        async move {
            if is_sending && !input.get().is_empty() {
                let input = input.get();
                let new_message = send_message(&input).await;

                set_message.write().insert(0, new_message);
                set_input.set(String::new());
                set_is_sending.set(false);
            }
        }
    });
    let input_handler = move |e: Targeted<Event, HtmlTextAreaElement>| {
        let value = e.target().value();

        set_input.set(value)
    };
    let keydown_handler = move |e: KeyboardEvent| {
        match e.key().as_str() {
            "Enter" => {
                if !e.shift_key() {
                    e.prevent_default();
                    set_is_sending.set(true);
                }
            }
            _ => (),
        };
    };

    view! {
        <div class="message-input-container">
            <textarea
                class="message-input"
                placeholder="Type your message..."
                on:input:target=input_handler
                on:keydown=keydown_handler
                prop:value=input
            />
            <button class="message-send" on:click=move |_| set_is_sending.set(true)>
                <Icon
                    style="height: 2rem; width: 2rem;"
                    icon=Signal::derive(move || {
                        if is_sending.get() {
                            icon::AiLoadingOutlined
                        } else {
                            icon::RiSendPlaneBusinessFill
                        }
                    })
                    style:animation=move || {
                        if is_sending.get() { "spin 1s linear infinite" } else { "None" }
                    }
                />

            </button>

        </div>
    }
}
