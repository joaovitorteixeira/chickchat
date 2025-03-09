use leptos::prelude::window;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            // TODO: it should get the user id from the server
            id: Self::generate_random_string(26),
        }
    }
}

impl User {
    pub fn get_user_from_session_storage() -> Self {
        let local_storage = window().local_storage().ok().flatten();
        let local_storage_clone = local_storage.clone();
        let user = local_storage
            .and_then(|storage| {
                storage
                    .get_item("user")
                    .ok()
                    .flatten()
                    .and_then(|value| serde_json::from_str::<User>(&value).ok())
            })
            .unwrap_or_else(|| {
                if let Some(storage) = local_storage_clone {
                    let new_user = User::default();
                    storage
                        .set_item("user", &serde_json::to_string(&new_user).unwrap())
                        .expect("Failed to save user to local storage");
                    new_user
                } else {
                    User::default()
                }
            });

        user
    }

    fn generate_random_string(length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }
}
