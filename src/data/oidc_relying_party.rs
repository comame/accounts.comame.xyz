use std::ops::Not;

use crate::db::relying_party::{
    add_redirect_uri, find_relying_party_by_id, register_relying_party,
};

#[derive(Eq, PartialEq, Debug)]
pub struct RelyingParty {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
}

impl RelyingParty {
    pub fn find(client_id: &str) -> Option<Self> {
        find_relying_party_by_id(client_id)
    }

    pub fn register(client_id: &str) -> Result<Self, ()> {
        register_relying_party(client_id)?;
        Ok(Self {
            client_id: client_id.to_string(),
            redirect_uris: vec![],
        })
    }

    pub fn add_redirect_uri(&self, redirect_uri: &str) -> Result<(), ()> {
        if redirect_uri.starts_with("https://").not() {
            return Err(());
        }
        add_redirect_uri(&self.client_id, redirect_uri)
    }
}
