use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::crypto::rand::random_str;
use crate::data::oidc_flow::oidc_scope::Scopes;
use crate::db::redis;

/// Authentication Endpoint でユーザーにログインさせる前後で必要な情報を保持しておく
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthenticationFlowState {
    id: String,
    pub relying_party_id: String,
    pub flow: OidcFlow,
    pub redirect_url: String,
    pub scopes: Scopes,
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum OidcFlow {
    Implicit,
    Code,
}

impl Display for OidcFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Implicit => write!(f, "implicit"),
            Self::Code => write!(f, "code"),
        }
    }
}

const PREFIX: &str = "AUTH_FLOW_STATE:";
const STATE_TIME: u64 = 5 * 60;

#[allow(clippy::too_many_arguments)]
impl AuthenticationFlowState {
    pub fn new(
        relying_party_id: &str,
        redirect_uri: &str,
        scopes: Scopes,
        state: Option<String>,
        nonce: Option<String>,
        max_age: Option<u64>,
        login_requirement: LoginRequirement,
        flow: OidcFlow,
    ) -> Self {
        let id = random_str(64);
        let state = Self {
            id,
            relying_party_id: relying_party_id.to_string(),
            redirect_url: redirect_uri.to_string(),
            scopes,
            state,
            nonce,
            max_age,
            login_requirement,
            flow,
        };

        redis::set(
            &format!("{PREFIX}{}", state.id()),
            &serde_json::to_string(&state).unwrap(),
            STATE_TIME,
        );

        return state;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn get_keep(id: &str) -> Option<AuthenticationFlowState> {
        let key = format!("{PREFIX}{id}");
        let result = redis::get(&key)?;
        Some(serde_json::from_str(&result).unwrap())
    }

    pub fn get_consume(id: &str) -> Option<AuthenticationFlowState> {
        let key = format!("{PREFIX}{id}");
        let result = redis::get(&key)?;
        redis::del(&key);
        Some(serde_json::from_str(&result).unwrap())
    }
}
