use message::{list_messages, send_message};
use rocket::response::Debug;

#[macro_use]
extern crate rocket;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

mod database;
mod message;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/chat", routes![send_message, list_messages])
        .attach(database::stage())
}
