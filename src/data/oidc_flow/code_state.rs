use serde::{Deserialize, Serialize};

use super::oidc_scope::Scopes;
use crate::crypto::rand::random_str;

#[derive(Serialize, Deserialize)]
pub struct CodeState {
    pub code: String,
    pub id_token: String,
    pub scope: Scopes,
}

impl CodeState {
    pub fn new(id_token: &str, scope: &Scopes) -> Self {
        let code = random_str(16);
        Self {
            code,
            id_token: id_token.to_string(),
            scope: scope.to_owned(),
        }
    }
}
