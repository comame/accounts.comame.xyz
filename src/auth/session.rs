use crate::{
    data::{session::Session, user::User},
    db::{
        session::{delete_by_token, delete_by_user, insert_session, select_session_by_token},
        user::find_user_by_id,
    },
};

pub fn create_session(user_id: &str) -> Session {
    let session = Session::new(user_id);
    insert_session(&session);
    session
}

pub fn revoke_session_by_user_id(user_id: &str) {
    delete_by_user(user_id);
}

pub fn revoke_session_by_token(token: &str) {
    delete_by_token(token);
}

pub fn get_session(token: &str) -> Option<User> {
    let session = select_session_by_token(token)?;
    let user = find_user_by_id(&session.user_id)?;
    Some(user)
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
        let user = get_session(&session.token);

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
    fn can_revoke_by_user_id() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-can-revoke-by-user";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let session = create_session(user_id);
        revoke_session_by_user_id(user_id);

        let user = get_session(&session.token);

        assert!(user.is_none());
    }

    #[test]
    fn can_revoke_by_token() {
        _test_init::init_mysql();
        _test_init::init_redis();

        let user_id = "session-can-revoke-by-token";
        db::user::insert_user(&User {
            id: String::from(user_id),
        })
        .unwrap();

        let session = create_session(user_id);
        revoke_session_by_token(&session.token);

        let user = get_session(&session.token);

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

        let user_1 = get_session(&session_1.token);
        let user_2 = get_session(&session_2.token);

        assert_eq!(user_1.unwrap().id, user_id);
        assert_eq!(user_2.unwrap().id, user_id);
    }
}
