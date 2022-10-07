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

// TODO: ユーザーの存在確認もする
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
    fn valid_password() {
        init_mysql();
        let user_id = "password-valid-password";
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        set_password(user_id, "foo");
        assert!(authenticated(user_id, "foo"));
    }

    #[test]
    fn invalid_password() {
        init_mysql();
        let user_id = "password-invalid-password";
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        set_password(user_id, "foo");
        assert!(!authenticated(user_id, "bar"));
        assert!(!authenticated("bob", "bar"));
    }
}
