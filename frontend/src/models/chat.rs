use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::util::env::Env;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub id: Option<String>,
    pub name: String,
}

impl Chat {
    pub async fn list(limit: u8, last_id: Option<String>) -> Vec<Chat> {
        let client = Client::new();
        let mut query = vec![("limit", limit.to_string())];

        if let Some(last_id) = last_id {
            query.push(("last_id", last_id));
        }

        let fetcher = client
            .get(format!("{}/chat", Env::get_backend_url()))
            .query(&query)
            .send().await;

        match fetcher {
            Ok(response) => response.json::<Vec<Chat>>().await.unwrap(),
            Err(_) => vec![],
        }
    }
}
