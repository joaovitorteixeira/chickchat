use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum MessageStatus {
    Sent,
    Failed,
}

#[derive(Clone, Debug)]
pub struct MessageWithStatus {
    pub message: Message,
    pub status: MessageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<String>,
    pub content:String,

    pub chat_id: String,
    pub user_id: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub is_sent: bool,
}

impl Message {
    pub fn new(content: String, user_id: String, chat_id: String) -> Self {
        Self { content, user_id, is_sent: false, chat_id, id: None }
    }

     pub async fn send(&mut self) -> MessageWithStatus {
        let client = Client::new();
        let fetcher = client
        // TODO: Change to env
            .post(format!("http://127.0.0.1:8000/chat/{}/message", self.chat_id))
            .json(&self)
            .send().await;
        
        self.is_sent = true;
            
        match fetcher {
            Ok(_) => {
                let created_message = fetcher.unwrap().json::<Message>().await.unwrap();
                
                self.id = created_message.id;

                MessageWithStatus{
                    message: self.clone(),
                    status: MessageStatus::Sent,
                }
            },
            Err(_) => MessageWithStatus{
                message: self.clone(),
                status: MessageStatus::Failed,
            },
        }
    }
}
