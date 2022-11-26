use crate::{
    crypto::rand::random_str,
    db::access_token::{get_access_token, insert_access_token},
};

use super::oidc_flow::oidc_scope::Scopes;

pub struct AccessToken {
    pub sub: String,
    pub scopes: Scopes,
    pub token: String,
}

impl AccessToken {
    pub fn new(sub: &str, scopes: &Scopes) -> Self {
        let v = Self {
            sub: sub.to_string(),
            scopes: scopes.to_owned(),
            token: random_str(32),
        };

        insert_access_token(&v);

        v
    }

    pub fn get(token: &str) -> Option<Self> {
        get_access_token(&token)
    }
}
