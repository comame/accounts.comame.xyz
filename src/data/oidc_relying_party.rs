use std::ops::Not;

use crate::{db::relying_party::{
    add_redirect_uri, find_relying_party_by_id, register_relying_party,
}, crypto::rand::random_str, auth::password::calculate_password_hash};

#[derive(Eq, PartialEq, Debug)]
pub struct RelyingParty {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub hashed_client_secret: String,
}

impl RelyingParty {
    pub fn find(client_id: &str) -> Option<Self> {
        find_relying_party_by_id(client_id)
    }

    /// Returns raw client_secret
    pub fn register(client_id: &str) -> Result<String, ()> {
        let client_secret = random_str(32);
        let hashed = calculate_password_hash(&client_secret, client_id);
        register_relying_party(client_id, &hashed)?;
        Ok(client_secret)
    }

    pub fn add_redirect_uri(&self, redirect_uri: &str) -> Result<(), ()> {
        if redirect_uri.starts_with("https://").not() {
            return Err(());
        }
        add_redirect_uri(&self.client_id, redirect_uri)
    }
}
