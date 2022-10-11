use crate::{crypto, time::now};

#[derive(Clone)]
pub struct Session {
    pub user_id: String,
    pub token: String,
    pub created_at: u64,
}

impl Session {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: String::from(user_id),
            token: crypto::rand::random_str(128),
            created_at: now(),
        }
    }
}
