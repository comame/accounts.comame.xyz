use serde::{Deserialize, Serialize};

use crate::db::userinfo::get_userinfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub sub: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

impl UserInfo {
    pub fn get(sub: &str) -> Option<Self> {
        get_userinfo(sub)
    }

    pub fn empty(sub: &str) -> Self {
        Self {
            sub: sub.to_string(),
            email: None,
            email_verified: None,
            name: None,
            preferred_username: None,
            profile: None,
            picture: None,
        }
    }
}
