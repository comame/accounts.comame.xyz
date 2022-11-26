use super::oidc_flow::oidc_scope::Scopes;
use crate::crypto::rand::random_str;
use crate::db::access_token::{get_access_token, insert_access_token};

pub const ACCESS_TOKEN_EXPIRES_IN: u16 = 3600;

pub struct AccessToken {
    pub sub: String,
    pub scopes: Scopes,
    pub token: String,
    pub expires_in: u16,
}

impl AccessToken {
    pub fn new(sub: &str, scopes: &Scopes) -> Self {
        let v = Self {
            sub: sub.to_string(),
            scopes: scopes.to_owned(),
            token: random_str(32),
            expires_in: ACCESS_TOKEN_EXPIRES_IN,
        };

        insert_access_token(&v);

        v
    }

    pub fn get(token: &str) -> Option<Self> {
        get_access_token(&token)
    }
}
