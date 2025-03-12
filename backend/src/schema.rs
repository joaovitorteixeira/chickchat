// @generated automatically by Diesel CLI.

diesel::table! {
    message (id, chat_id) {
        #[max_length = 26]
        id -> Char,
        content -> Text,
        #[max_length = 26]
        chat_id -> Char,
        #[max_length = 26]
        user_id -> Char,
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

diesel::allow_tables_to_appear_in_same_query!(
    message,
    user,
);
