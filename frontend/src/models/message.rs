use leptos::{prelude::{Write, WriteSignal}, wasm_bindgen::{prelude::Closure, JsCast}, web_sys::{EventSource, MessageEvent}};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::user::User;

#[derive(Clone, Debug)]
pub enum MessageStatus {
    Sent,
    Failed,
}

#[derive(Clone, Debug)]
pub struct MessageWithStatus {
    pub message: Message,
    pub _status: MessageStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
                    _status: MessageStatus::Sent,
                }
            },
            Err(_) => MessageWithStatus{
                message: self.clone(),
                _status: MessageStatus::Failed,
            },
        }
    }

    pub async fn list(limit: u8, last_id: Option<String>, chat_id: String) -> Vec<MessageWithStatus> {
        let client = Client::new();
        let mut query = vec![("limit", limit.to_string())];

        if let Some(last_id) = last_id {
            query.push(("last_id", last_id));
        }

        let messages = client.get(format!("http://127.0.0.1:8000/chat/{}/message",chat_id))
        .query(&query).send().await.unwrap().json::<Vec<Message>>().await.unwrap();

        messages.iter().map(|message| MessageWithStatus{message: message.clone(), _status: MessageStatus::Sent}).collect()
    }

    pub fn on_message(channel_id: String, set_messages: WriteSignal<Vec<MessageWithStatus>>) {
        let chat_id = channel_id.clone();
        let event_source = EventSource::new(&format!("http://127.0.0.1:8000/chat/{}/event", chat_id)).unwrap();
        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            let data = event.data().as_string().unwrap();
            let message = MessageWithStatus {
                message: serde_json::from_str(&data).unwrap(),
                _status: MessageStatus::Sent,
            };
            let user = User::get_user_from_session_storage();
            
            if message.message.user_id != user.id {
                set_messages.write().insert(0, message);
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        event_source.add_event_listener_with_callback("message", on_message.as_ref().unchecked_ref()).unwrap();
        on_message.forget();
    }
}
