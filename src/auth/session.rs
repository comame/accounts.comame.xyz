use crate::{
    crypto::rand::random_str,
    data::{session::Session, user::User},
    db::{redis, user::find_user_by_id},
};

const TOKEN_TO_USER: &str = "SESSION-TOKEN-";
const USER_TO_TOKEN: &str = "SESSION-USER-";

pub fn create_session(user_id: &str) -> Session {
    let session = Session::new(user_id);

    let redis_key = String::from(TOKEN_TO_USER) + &session.token();
    redis::set(&redis_key, user_id, 24 * 60 * 60);

    let rand = random_str(16);
    let redis_key = format!("{}{}-{}", USER_TO_TOKEN, user_id, rand);
    redis::set(&redis_key, &session.token(), 24 * 60 * 60);

    session
}

pub fn revoke_session(user_id: &str) {
    let key_pattern = format!("{}{}*", USER_TO_TOKEN, user_id);
    let keys = redis::list_keys_pattern(&key_pattern);

    let mut tokens: Vec<String> = vec![];
    for key in keys {
        let key = redis::get(&key);
        if key.is_none() {
            continue;
        }
        tokens.push(key.unwrap());
    }

    for token in tokens {
        let key = format!("{}{}", TOKEN_TO_USER, token);
        redis::del(&key);
    }
}

pub fn get_session(token: &str) -> Option<User> {
    let redis_key = String::from(TOKEN_TO_USER) + token;
    let user = redis::get(&redis_key);

    if user.is_none() {
        return None;
    }

    let user = find_user_by_id(&user.unwrap());

    if user.is_none() {
        return None;
    }

    Some(user.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::db::_test_init;

    #[test]
    fn can_get_session() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-can-get";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let session = create_session(user_id);
        let user = get_session(&session.token());

        assert_eq!(user_id, user.unwrap().id);
    }

    #[test]
    fn fail_session() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-fail-session";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let _session = create_session(user_id);
        let user = get_session("dummy_session");

        assert!(user.is_none());
    }

    #[test]
    fn can_revoke() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-can-revoke";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let session = create_session(user_id);
        revoke_session(user_id);

        let user = get_session(session.token());

        assert!(user.is_none());
    }

    #[test]
    fn can_create_multi_session() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-create-multi-session";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let session_1 = create_session(user_id);
        let session_2 = create_session(user_id);

        let user_1 = get_session(session_1.token());
        let user_2 = get_session(session_2.token());

        assert_eq!(user_1.unwrap().id, user_id);
        assert_eq!(user_2.unwrap().id, user_id);
    }
}
