use crate::{crypto::rand, db::redis};
use std::ops::Not;

const PREFIX: &str = "csrf-token-";
const TOKEN_ALIVE_MIN: u16 = 10;

pub fn generate() -> String {
    let token = rand::random_str(32);

    let redis_key = String::from(PREFIX) + &token;
    redis::set(&redis_key, "", TOKEN_ALIVE_MIN * 60);

    token
}

pub fn validate(token: &str) -> bool {
    let redis_key = String::from(PREFIX) + token;
    let is_collect = redis::list_keys_pattern(&redis_key).is_empty().not();
    redis::del(&redis_key);
    is_collect
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::_test_init::init_redis;

    #[test]
    fn test() {
        init_redis();
        let token = generate();
        assert!(validate(&token));
        assert!(!validate(&token));
    }
}
