use super::session::revoke_session_by_user_id;
use crate::crypto::sha::sha256;
use crate::data::authentication::{Authentication, AuthenticationMethod, LoginPrompt};
use crate::data::user_password::UserPassword;
use crate::db::user;
use crate::db::user_password::{insert_password, password_matched};
use crate::time::now;

pub fn calculate_password_hash(password: &str, salt: &str) -> String {
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

    revoke_session_by_user_id(user_id);

    if password.is_empty() {
        return;
    }

    let user_password = UserPassword {
        user_id: user_id.to_string(),
        hashed_password: calculate_password_hash(password, user_id),
    };
    insert_password(&user_password).unwrap();
}

pub fn authenticate(user_id: &str, password: &str, audience: &str, _prompt: LoginPrompt) -> bool {
    let hash = calculate_password_hash(password, user_id);
    let user_password = UserPassword {
        user_id: user_id.to_string(),
        hashed_password: hash,
    };

    let password_ok = password_matched(&user_password);
    let user_found = user::find_user_by_id(user_id).is_some();

    if !password_ok {
        return false;
    }

    if !user_found {
        return false;
    }

    Authentication::create(now(), audience, user_id, AuthenticationMethod::Password);

    password_ok && user_found
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::user::User;
    use crate::db::_test_init::init_mysql;
    use crate::db::user::insert_user;
    use crate::db::{self};

    #[test]
    fn valid_password() {
        init_mysql();
        let user_id = "password-valid-password";
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        set_password(user_id, "foo");
        assert!(authenticate(
            user_id,
            "foo",
            "aud.comame.dev",
            LoginPrompt::Login
        ));
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
        assert!(!authenticate(
            user_id,
            "bar",
            "aud.comame.dev",
            LoginPrompt::Login
        ));
        assert!(!authenticate(
            "bob",
            "bar",
            "aud.comame.dev",
            LoginPrompt::Login
        ));
    }

    #[test]
    #[should_panic]
    fn invalid_user() {
        init_mysql();
        let user_id = "auth-password-invalid_user-alice";
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        // Panic here because user is not exist
        db::user_password::insert_password(&UserPassword {
            user_id: "auth-password-invalid_user-user_not_exists".to_string(),
            hashed_password: "dummy".to_string(),
        })
        .unwrap();
    }
}
