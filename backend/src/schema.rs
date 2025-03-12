// @generated automatically by Diesel CLI.

diesel::table! {
    chat (id) {
        #[max_length = 26]
        id -> Char,
        created_at -> Timestamp,
        #[max_length = 32]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    chat_member (id) {
        #[max_length = 26]
        id -> Char,
        #[max_length = 26]
        chat_id -> Char,
        #[max_length = 26]
        user_id -> Char,
    }
}

diesel::table! {
    message (id, chat_id) {
        #[max_length = 26]
        id -> Char,
        content -> Text,
        #[max_length = 26]
        chat_id -> Char,
        #[max_length = 26]
        member_id -> Char,
    }
}

diesel::table! {
    user (id) {
        #[max_length = 26]
        id -> Char,
        #[max_length = 32]
        username -> Varchar,
    }
}

diesel::joinable!(chat_member -> chat (chat_id));
diesel::joinable!(chat_member -> user (user_id));
diesel::joinable!(message -> chat (chat_id));
diesel::joinable!(message -> chat_member (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    chat,
    chat_member,
    message,
    user,
);
