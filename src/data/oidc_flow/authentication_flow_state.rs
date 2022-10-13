use serde::{Deserialize, Serialize};

use crate::crypto::rand::random_str;


#[derive(Serialize, Deserialize, Clone)]
pub struct AuthenticationFlowState {
    id: String,
    pub relying_party_id: String,
    pub redirect_url: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub max_age: Option<u64>,
    pub login_requirement: LoginRequirement,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LoginRequirement {
    // ユーザーの確認が必要
    Consent,
    // 再認証が必要
    ReAuthenticate,
    // 前回の認証から max_age 秒過ぎていたら再認証
    MaxAge,
    // 認証画面を出してはならない
    None,
    // なんでもいい
    Any,
}

impl LoginRequirement {
    pub fn parse(str: &str) -> Result<Self, ()> {
        match str {
            "consent" => Ok(Self::Consent),
            "reauthenticate" => Ok(Self::ReAuthenticate),
            "max_age" => Ok(Self::MaxAge),
            "none" => Ok(Self::None),
            "any" => Ok(Self::Any),
            _ => Err(()),
        }
    }
}

impl AuthenticationFlowState {
    pub fn new(
        relying_party_id: &str,
        redirect_uri: &str,
        state: Option<String>,
        nonce: Option<String>,
        max_age: Option<u64>,
        login_requirement: LoginRequirement,
    ) -> Self {
        let id = random_str(64);
        Self {
            id,
            relying_party_id: relying_party_id.to_string(),
            redirect_url: redirect_uri.to_string(),
            state,
            nonce,
            max_age,
            login_requirement,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
