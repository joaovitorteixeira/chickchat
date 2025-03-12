use std::collections::HashMap;

use chat::create_chat;
use cors::CORS;
use message::{get_message_event, list_messages, send_message, Message};
use rocket::{
    response::Debug,
    tokio::sync::{broadcast::Sender, RwLock},
};

#[macro_use]
extern crate rocket;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

mod chat;
mod cors;
mod database;
mod message;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(RwLock::new(HashMap::<String, Sender<Message>>::new()))
        .mount(
            "/chat",
            routes![create_chat, send_message, list_messages, get_message_event],
        )
        .attach(database::stage())
        .attach(CORS)
}
