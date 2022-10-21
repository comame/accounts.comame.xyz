use crate::crypto::rand::random_str;
use crate::data::user::User;
use crate::db::external_session::{get_session, insert_session};
use crate::time::now;

#[derive(Clone)]
pub struct ExternalSession {
    pub client_id: String,
    pub token: String,
    pub user_id: String,
    pub created_at: u64,
}

impl ExternalSession {
    pub fn create(client_id: &str, user_id: &str) -> Result<Self, ()> {
        let token = random_str(64);
        let user = User::find(user_id);
        if user.is_none() {
            return Err(());
        }
        let session = Self {
            client_id: client_id.to_string(),
            token,
            user_id: user_id.to_string(),
            created_at: now(),
        };
        insert_session(&session);
        Ok(session)
    }

    pub fn get(client_id: &str, token: &str) -> Option<Self> {
        get_session(client_id, token)
    }
}
