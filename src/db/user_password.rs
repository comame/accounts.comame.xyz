use super::mysql::get_conn;
use crate::data::user_password::UserPassword;
use mysql::prelude::*;
use mysql::{params, Error};

pub fn password_matched(user_password: &UserPassword) -> bool {
    let result = get_conn().unwrap().exec_map(
        "SELECT user_id FROM user_passwords WHERE user_id = :user_id AND hashed_password = :hashed_password",
        params! {
            "user_id" => user_password.user_id.clone(),
            "hashed_password" => user_password.hashed_password.clone(),
        },
        |(_user_id,): (String,)| 0
    ).unwrap();

    !result.is_empty()
}

fn password_exists(user_id: &str) -> bool {
    let result = get_conn()
        .unwrap()
        .exec_map(
            "SELECT user_id FROM user_passwords WHERE user_id = :user_id",
            params! {
                "user_id" => user_id,
            },
            |(_user_id,): (String,)| 0,
        )
        .unwrap();

    !result.is_empty()
}

pub fn insert_password(user_password: &UserPassword) -> Result<(), Error> {
    let exists = password_exists(&user_password.user_id);

    if exists {
        get_conn().unwrap().exec_batch(
            "UPDATE user_passwords SET hashed_password = :new_p WHERE user_id = :id",
            std::iter::once(params! {
                "new_p" => user_password.hashed_password.clone(),
                "id" => user_password.user_id.clone(),
            }),
        )
    } else {
        get_conn().unwrap().exec_batch(
            "INSERT INTO user_passwords (user_id, hashed_password) VALUES (:id, :pass)",
            std::iter::once(params! {
                "id" => user_password.user_id.clone(),
                "pass" => user_password.hashed_password.clone(),
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::_test_init::init;
    use super::*;
    use crate::data::user_password::UserPassword;

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_insert_password() {
        init();
        let pass = UserPassword {
            user_id: "user-a".to_string(),
            hashed_password: "pass".to_string(),
        };
        let result = insert_password(&pass);
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_can_authenticate() {
        init();
        let pass_1 = UserPassword {
            user_id: "user-a".to_string(),
            hashed_password: "pass".to_string(),
        };
        insert_password(&pass_1).unwrap();
        let result = password_matched(&pass_1);
        assert!(result);
        let pass_2 = UserPassword {
            user_id: "user-a".to_string(),
            hashed_password: "wrong".to_string(),
        };
        let result = password_matched(&pass_2);
        assert!(!result);
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_can_update() {
        init();
        let pass_1 = UserPassword {
            user_id: "user-a".to_string(),
            hashed_password: "pass".to_string(),
        };
        insert_password(&pass_1).unwrap();
        let pass_2 = UserPassword {
            user_id: "user-a".to_string(),
            hashed_password: "new".to_string(),
        };
        insert_password(&pass_2).unwrap();
        assert!(!password_matched(&pass_1));
        assert!(password_matched(&pass_2));
    }
}
