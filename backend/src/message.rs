use std::collections::HashMap;

use crate::{
    database::DbConn, schema::message, util::pagination::PaginationRequest, Result
};
use rocket::{
     response::{
        status::Created,
        stream::{Event, EventStream},
    }, serde::{json::Json, Deserialize, Serialize}, tokio::{
        select,
        sync::{broadcast::{error::RecvError, Sender}, RwLock},
    }, Shutdown, State
};
use rocket_db_pools::{diesel::prelude::*, Connection};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = message)]
pub struct Message {
    #[serde(skip_deserializing)]
    pub id: String,
    pub content: String,
    #[serde(skip_deserializing)]
    pub chat_id: String,
    pub member_id: String,
}

#[post("/<chat_id>/message", data = "<message>")]
pub async fn send_message(
    chat_id: &str,
    mut message: Json<Message>,
    mut db_conn: Connection<DbConn>,
    channel_map: &State<RwLock<HashMap<String, Sender<Message>>>>,
) -> Result<Created<Json<Message>>> {
    let queue = get_channel(chat_id, channel_map).await;

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

    let _res = queue.send(message.clone().into_inner());

    Ok(Created::new(format!("/chat/{}/message/{}", chat_id, message.id)).body(message))
}

#[get("/<chat_id>/message?<list_query..>")]
pub async fn list_messages(
    chat_id: &str,
    mut db_conn: Connection<DbConn>,
    list_query: PaginationRequest,
) -> Result<Json<Vec<Message>>> {
    let mut query = message::table
        .filter(message::chat_id.eq(chat_id))
        .into_boxed();
    
    // TODO - How can I turn this code into a generic pagination function?
    if let Some(last_id) = list_query.last_id {
        query = query.filter(message::id.lt(last_id));
    }

    let messages = query
        .select(message::all_columns)
        .order(message::id.desc())
        .limit(list_query.limit.into())
        .load(&mut db_conn)
        .await?;

    Ok(Json(messages))
}

#[get("/<chat_id>/event")]
pub async fn get_message_event(
    chat_id: &str,
    chat_map: &State<RwLock<HashMap<String, Sender<Message>>>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = get_channel(chat_id, chat_map).await.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

async fn get_channel(chat_id: &str, chat_map: &State<RwLock<HashMap<String, Sender<Message>>>>) -> Sender<Message> {
    {
        let read_guard = chat_map.read().await;
        if let Some(sender) = read_guard.get(chat_id) {
            return sender.clone();
        }
    }

    let mut write_guard = chat_map.write().await;
    let sender = Sender::new(1024);
    write_guard.insert(chat_id.to_string(), sender.clone());

    sender
}