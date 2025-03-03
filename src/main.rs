use database::DbConn;
use model::Message;
use rocket::response::status::Created;
use rocket::response::Debug;
use rocket::serde::json::Json;
use rocket_db_pools::diesel::prelude::{AsyncConnection, RunQueryDsl};
use rocket_db_pools::Connection;
use schema::message;

#[macro_use]
extern crate rocket;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

mod database;
mod model;
pub mod schema;

#[post("/<chat_id>/message", data = "<message>")]
async fn send_message(
    chat_id: &str,
    mut message: Json<Message>,
    mut db_conn: Connection<DbConn>,
) -> Result<Created<Json<Message>>> {
    message.chat_id = chat_id.to_string();
    message.id = ulid::Ulid::new().to_string();

    let message = db_conn
        .transaction(|mut conn| {
            Box::pin(async move {
                diesel::insert_into(message::table)
                    .values(&*message)
                    .execute(&mut conn)
                    .await?;

                Ok::<_, diesel::result::Error>(message)
            })
        })
        .await?;

    Ok(Created::new(format!("/chat/{}/message/{}", chat_id, message.id)).body(message))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/chat", routes![send_message])
        .attach(database::stage())
}
