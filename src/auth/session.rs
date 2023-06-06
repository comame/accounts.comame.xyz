
use crate::data::session::Session;
use crate::data::user::User;
use crate::db::session::{
    delete_by_token, delete_by_user, insert_session, select_session_by_token,
};
use crate::db::user::find_user_by_id;

const SESSION_EXPIRE_MIN: u64 = 24 * 60;

pub fn create_session(user_id: &str) -> Option<Session> {
    let user = User::find(user_id);
    user.as_ref()?;
    let session = Session::new(user_id);
    insert_session(&session);
    Some(session)
}

pub fn revoke_session_by_user_id(user_id: &str) {
    delete_by_user(user_id);
}

pub fn revoke_session_by_token(token: &str) {
    delete_by_token(token);
}

pub fn authenticate(_audience: &str, token: &str) -> Option<User> {
    if token.is_empty() {
        return None;
    }

    let session = select_session_by_token(token, SESSION_EXPIRE_MIN);

    session.as_ref()?;

    let session = session.unwrap();
    let user_id = session.user_id;
    let _created_at = session.created_at;

    let user = find_user_by_id(&user_id);

    user.as_ref()?;

    let user = user.unwrap();

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
        let user = authenticate("aud.comame.dev", &session.unwrap().token);

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
        let user = authenticate("aud.comame.dev", "dummy_session");

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

        let user = authenticate("aud.comame.dev", &session.unwrap().token);

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

        let session = create_session(user_id).unwrap();
        revoke_session_by_token(&session.token);

        let user = authenticate("aud.comame.dev", &session.token);

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

        let user_1 = authenticate("aud.comame.dev", &session_1.unwrap().token);
        let user_2 = authenticate("aud.comame.dev", &session_2.unwrap().token);

        assert_eq!(user_1.unwrap().id, user_id);
        assert_eq!(user_2.unwrap().id, user_id);
    }
}
