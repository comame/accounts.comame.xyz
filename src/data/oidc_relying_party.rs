

use serde::Serialize;



use crate::db::relying_party::{
    find_relying_party_by_id,
};

#[derive(Eq, PartialEq, Debug, Serialize)]
pub struct RelyingParty {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub hashed_client_secret: String,
}

impl RelyingParty {
    pub fn find(client_id: &str) -> Option<Self> {
        find_relying_party_by_id(client_id)
    }
}
