use crate::crypto;

pub struct Session {
    user_id: String,
    token: String,
}

impl Session {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: String::from(user_id),
            token: crypto::rand::random_str(128),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}
