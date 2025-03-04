use message::{get_message_event, list_messages, send_message, Message};
use rocket::{response::Debug, tokio::sync::broadcast::channel};

#[macro_use]
extern crate rocket;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

mod database;
mod message;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount(
            "/chat",
            routes![send_message, list_messages, get_message_event],
        )
        .attach(database::stage())
}
