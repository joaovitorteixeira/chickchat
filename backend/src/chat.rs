use std::ops::RangeInclusive;
use rocket::{form::{self, Error, FromForm}, response::status::Created, serde::{json::Json, Deserialize, Serialize}};
use rocket_db_pools::{Connection, diesel::prelude::*};

use crate::{database::DbConn, schema::chat, Result};

fn is_option_string<'v>(value: &Option<String>, range: RangeInclusive<usize>) -> form::Result<'v, ()> {
    if let Some(value) = value {
        if !range.contains(&value.len()) {
            return Err(Error::validation(format!("length must be {} to {} characters", range.start(), range.end())))?;
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, FromForm)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = chat)]
pub struct Chat {
    #[serde(skip_deserializing)]
    pub id: String,

    #[field(validate = is_option_string(1..=32))]
    pub name: Option<String>,
}

#[post("/", data = "<chat>")]
pub async fn create_chat(mut chat: Json<Chat>, mut db_conn: Connection<DbConn>) -> Result<Created<Json<Chat>>>  {
    chat.id = ulid::Ulid::new().to_string();
    diesel::insert_into(chat::table).values(&*chat).execute(&mut db_conn).await?;

    Ok(Created::new(format!("/chat/{}", chat.id)).body(chat))
}
