use rocket_db_pools::diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use crate::schema::message;

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