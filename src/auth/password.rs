use crate::crypto::sha::sha256;

pub fn calculate_password_hash(password: &str, salt: &str) -> String {
    let with_salt = password.to_string() + salt;
    sha256(with_salt.as_str())
}
