use leptos::{
    ev::Targeted,
    prelude::*,
    web_sys::{Event, HtmlTextAreaElement, KeyboardEvent},
};

#[component]
pub fn MessageInput(set_message: WriteSignal<Vec<String>>) -> impl IntoView {
    let (input, set_input) = signal(String::new());
    let send_message = move || {
        if !input.get().is_empty() {
            set_message.write().push(input.get());
            set_input.set(String::new());
        }
    };
    let input_handler = move |e: Targeted<Event, HtmlTextAreaElement>| {
        let value = e.target().value();

        set_input.set(value)
    };
    let keydown_handler = move |e: KeyboardEvent| {
        match e.key().as_str() {
            "Enter" => {
                if !e.shift_key() {
                    e.prevent_default();
                    send_message();
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
            <button class="message-send" on:click=move |_| send_message()>
                "Send"
            </button>
        </div>
    }
}
