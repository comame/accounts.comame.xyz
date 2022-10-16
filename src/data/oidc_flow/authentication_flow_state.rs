use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::crypto::rand::random_str;
use crate::data::oidc_flow::oidc_scope::Scopes;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Clone, Serialize, Deserialize)]
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

impl OidcFlow {
    fn parse(str: &str) -> Result<Self, ()> {
        match str {
            "implicit" => Ok(Self::Implicit),
            "code" => Ok(Self::Code),
            _ => Err(()),
        }
    }
}

#[warn(clippy::too_many_arguments)]
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
        Self {
            id,
            relying_party_id: relying_party_id.to_string(),
            redirect_url: redirect_uri.to_string(),
            scopes,
            state,
            nonce,
            max_age,
            login_requirement,
            flow,
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
