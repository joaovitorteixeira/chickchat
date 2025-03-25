use leptos::prelude::*;
use leptos_router::hooks::use_query;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{components::chat_list::ChatQueryParam, util::env::Env};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Chat {
    pub id: Option<String>,
    pub name: String,
}

impl Chat {
    pub fn new(name: String) -> Self {
        Self { name, id: None }
    }

    pub async fn create(self) -> Result<Self, reqwest::Error> {
        let client = Client::new();
        let fetcher = client
        .post(format!("{}/chat", Env::get_backend_url()))
        .json(&self)
        .send().await;

        let created_chat = fetcher?.json::<Chat>().await?;
        Ok(Self {
            id: created_chat.id,
            name: self.name,
        })
    }

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

    pub fn get_id_from_query() -> Memo<Option<String>> {
        let query = use_query::<ChatQueryParam>();
        
        Memo::new(move |_| query.read().as_ref().ok().and_then(|q| q.chat_id.clone()))
    }
}
