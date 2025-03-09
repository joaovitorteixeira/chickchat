use crate::{
    database::DbConn,
    schema::message::{self, id},
    Result,
};
use rocket::{
    form::{self, FromForm, Error}, response::{
        status::Created,
        stream::{Event, EventStream},
    }, serde::{json::Json, Deserialize, Serialize}, tokio::{
        select,
        sync::broadcast::{error::RecvError, Sender},
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
    pub user_id: String,
}

fn validate_ulid_length<'v>(value: &Option<String>) -> form::Result<'v, ()> {
    if let Some(value) = value {
        if value.len() != ulid::ULID_LEN {
            Err(Error::validation("length must be 26 characters"))?;
        }
    }

    Ok(())
    
}

#[derive(FromForm)]
pub struct ListMessage {
    #[field(validate = range(1..=100))]
    limit: u8,

    #[field(validate = validate_ulid_length())]
    last_id: Option<String>,
}

#[post("/<chat_id>/message", data = "<message>")]
pub async fn send_message(
    chat_id: &str,
    mut message: Json<Message>,
    mut db_conn: Connection<DbConn>,
    queue: &State<Sender<Message>>,
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

    let _res = queue.send(message.clone().into_inner());

    Ok(Created::new(format!("/chat/{}/message/{}", chat_id, message.id)).body(message))
}

#[get("/<chat_id>/message?<list_query..>")]
pub async fn list_messages(
    chat_id: &str,
    mut db_conn: Connection<DbConn>,
    list_query: ListMessage,
) -> Result<Json<Vec<Message>>> {
    let mut query = message::table
        .filter(message::chat_id.eq(chat_id))
        .into_boxed();

    if let Some(last_id) = list_query.last_id {
        query = query.filter(message::id.lt(last_id));
    }

    let messages = query
        .select(message::all_columns)
        .order(id.desc())
        .limit(list_query.limit.into())
        .load(&mut db_conn)
        .await?;

    Ok(Json(messages))
}

#[get("/event")]
pub async fn get_message_event(
    queue: &State<Sender<Message>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();

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
