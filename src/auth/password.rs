use crate::crypto::sha::sha256;
use crate::data::user_password::UserPassword;
use crate::db::user;
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
    let user_exists = user::find_user_by_id(user_id).is_some();
    if !user_exists {
        return;
    }

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
    let password_ok = password_matched(&user_password);
    if !password_ok {
        return false;
    }

    

    user::find_user_by_id(user_id).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::user::User,
        db::{self, _test_init::init_mysql, user::insert_user},
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

    #[test]
    fn invalid_user() {
        init_mysql();
        let user_id = "auth-password-invalid_user-alice";
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        db::user_password::insert_password(&UserPassword {
            user_id: "auth-password-invalid_user-user_not_exists".to_string(),
            hashed_password: "dummy".to_string(),
        })
        .unwrap();
        assert!(!authenticated("alice", "foo"));
    }
}
