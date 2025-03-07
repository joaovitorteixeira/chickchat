use leptos::prelude::*;

#[component]
pub fn MessageList() -> impl IntoView {
    let messages = vec![
        "Hello, how are you?",
        "I'm fine, thank you!",
        "What about you?",
        "I'm doing well, thanks for asking.",
        "Have you heard about the new project?",
        "Yes, it sounds really interesting!",
        "I can't wait to get started.",
        "Me too, it's going to be great.",
        "Do you have any plans for the weekend?",
        "Not yet, do you?",
        "I might go hiking if the weather is nice.",
        "That sounds like fun!",
        "Hello, how are you?",
        "I'm fine, thank you!",
        "What about you?",
        "I'm doing well, thanks for asking.",
        "Have you heard about the new project?",
        "Yes, it sounds really interesting!",
        "I can't wait to get started.",
        "Me too, it's going to be great.",
        "Do you have any plans for the weekend?",
        "Not yet, do you?",
        "I might go hiking if the weather is nice.",
        "That sounds like fun!",
    ];

    view! {
        <div class="message-list">
            {messages
                .iter()
                .enumerate()
                .map(|(index, message)| {
                    view! {
                        <div
                            class="message-item"
                            style:align-self=if index % 2 == 0 { "flex-start" } else { "flex-end" }
                        >
                            {*message}
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
