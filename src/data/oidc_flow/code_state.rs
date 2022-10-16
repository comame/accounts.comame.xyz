use serde::{Deserialize, Serialize};

use super::oidc_scope::Scopes;
use crate::crypto::rand::random_str;

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeState {
    pub code: String,
    pub client_id: String,
    pub id_token: String,
    pub scope: Scopes,
    pub redirect_uri: String,
}

impl CodeState {
    pub fn new(id_token: &str, client_id: &str, scope: &Scopes, redirect_uri: &str) -> Self {
        let code = random_str(16);
        Self {
            code,
            id_token: id_token.to_string(),
            client_id: client_id.to_string(),
            scope: scope.to_owned(),
            redirect_uri: redirect_uri.to_owned(),
        }
    }
}
