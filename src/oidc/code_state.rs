use serde_json::{from_str, to_string};

use crate::data::oidc_flow::code_state::CodeState;
use crate::db::redis;

const PREFIX: &str = "CODE_STATE:";
const STATE_TIME: u64 = 5 * 60;

pub fn save_state(state: &CodeState) {
    redis::set(
        &format!("{PREFIX}{}", state.code),
        &to_string(&state).unwrap(),
        STATE_TIME,
    );
}

pub fn get_state(code: &str) -> Option<CodeState> {
    let key = format!("{PREFIX}{code}");
    let result = redis::get(code)?;
    redis::del(&key);
    Some(from_str(&result).unwrap())
}
