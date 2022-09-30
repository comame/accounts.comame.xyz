use crate::crypto::sha::sha256;
use crate::db::user_password::password_matched;
use crate::data::user_password::UserPassword;

pub fn calculate_password_hash(password: &str, salt: &str) -> String {
    let with_salt = password.to_string() + salt;
    let mut hash = String::new();
    for _i in 0..3 {
        hash = sha256(with_salt.as_str())
    }
    hash
}

pub fn authenticated(user_id: &str, password: &str) -> bool {
    let hash = calculate_password_hash(password, user_id);
    let user_password = UserPassword { user_id: user_id.to_string(), hashed_password: hash };
    password_matched(&user_password)
}
