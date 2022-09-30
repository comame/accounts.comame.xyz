use crate::crypto::sha::sha256;
use crate::data::user_password::UserPassword;
use crate::db::user_password::{insert_password, password_matched};

fn calculate_password_hash(password: &str, salt: &str) -> String {
    let with_salt = password.to_string() + salt;
    let mut hash = String::new();
    for _i in 0..3 {
        hash = sha256(with_salt.as_str())
    }
    hash
}

pub fn set_password(user_id: &str, password: &str) {
    let user_password = UserPassword {
        user_id: user_id.to_string(),
        hashed_password: calculate_password_hash(password, user_id),
    };
    insert_password(&user_password).unwrap();
}

pub fn authenticated(user_id: &str, password: &str) -> bool {
    let hash = calculate_password_hash(password, user_id);
    let user_password = UserPassword {
        user_id: user_id.to_string(),
        hashed_password: hash,
    };
    password_matched(&user_password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::user::User,
        db::{_test_init::init_mysql, user::insert_user},
    };

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_valid_password() {
        init_mysql();
        insert_user(&User {
            id: "user".to_string(),
        })
        .unwrap();
        set_password("user", "foo");
        assert!(authenticated("user", "foo"));
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_invalid_password() {
        init_mysql();
        insert_user(&User {
            id: "user".to_string(),
        })
        .unwrap();
        set_password("user", "foo");
        assert!(!authenticated("user", "bar"));
        assert!(!authenticated("bob", "bar"));
    }
}
