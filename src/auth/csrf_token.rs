use std::ops::Not;

use crate::crypto::rand;
use crate::db::redis;

const PREFIX: &str = "csrf-token-";
const TOKEN_ALIVE_MIN: u64 = 10;

pub fn generate() -> String {
    let token = rand::random_str(32);

    let redis_key = String::from(PREFIX) + &token;
    redis::set(&redis_key, "", TOKEN_ALIVE_MIN * 60);

    token
}

pub fn validate_once(token: &str) -> bool {
    let is_collect = validate_keep_token(token);
    let redis_key = String::from(PREFIX) + token;
    redis::del(&redis_key);
    is_collect
}

pub fn validate_keep_token(token: &str) -> bool {
    let redis_key = String::from(PREFIX) + token;
    redis::list_keys_pattern(&redis_key).is_empty().not()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::_test_init::init_redis;

    #[test]
    fn test_once() {
        init_redis();
        let token = generate();
        assert!(validate_once(&token));
        assert!(!validate_once(&token));
    }

    #[test]
    fn test_keep() {
        init_redis();
        let token = generate();
        assert!(validate_keep_token(&token));
        assert!(validate_keep_token(&token));
    }
}
