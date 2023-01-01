use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};


use crate::data::openid_provider::OpenIDProvider;
use crate::db::redis;

const REDIS_PREFIX: &str = "relying_party_state:";

#[derive(Serialize, Deserialize)]
pub struct RelyingPartyState {
    /// `AuthenticationFlowState.id` と同じ。Authorization Request 以外から飛んでくることはありえないため
    pub state_id: String,
    pub nonce: String,
    pub state: String,
    pub op: OpenIDProvider,
    pub user_agent_id: String,
}

impl RelyingPartyState {
    pub fn save(v: &Self) {
        let v = v.clone();
        let redis_key = format!("{REDIS_PREFIX}{}", v.state_id);
        let redis_value = to_string(&v).unwrap();

        redis::set(&redis_key, &redis_value, 300);
    }

    pub fn get_keep(state_id: &str) -> Option<Self> {
        let redis_key = format!("{REDIS_PREFIX}{state_id}");
        let value = redis::get(&redis_key)?;
        Some(from_str::<Self>(&value).unwrap())
    }

    pub fn get_consume(state_id: &str) -> Option<Self> {
        let result = Self::get_keep(state_id)?;
        let state_id = result.state_id.clone();
        let redis_key = format!("{REDIS_PREFIX}{state_id}");
        redis::del(&redis_key);
        Some(result)
    }
}
