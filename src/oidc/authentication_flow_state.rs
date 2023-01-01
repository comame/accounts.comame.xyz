use serde_json::{from_str, to_string};

use crate::data::oidc_flow::authentication_flow_state::AuthenticationFlowState;
use crate::db::redis;

const PREFIX: &str = "AUTH_FLOW_STATE:";
const STATE_TIME: u64 = 5 * 60;

pub fn save_state(state: AuthenticationFlowState) {
    redis::set(
        &format!("{PREFIX}{}", state.id()),
        &to_string(&state).unwrap(),
        STATE_TIME,
    );
}

pub fn get_state_keep(id: &str) -> Option<AuthenticationFlowState> {
    let key = format!("{PREFIX}{id}");
    let result = redis::get(&key)?;
    Some(from_str(&result).unwrap())
}

pub fn get_state_consume(id: &str) -> Option<AuthenticationFlowState> {
    let key = format!("{PREFIX}{id}");
    let result = redis::get(&key)?;
    redis::del(&key);
    Some(from_str(&result).unwrap())
}
