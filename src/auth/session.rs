use crate::data::authentication::{Authentication, AuthenticationMethod, LoginPrompt};
use crate::data::authentication_failure::{AuthenticationFailure, Reason};
use crate::data::session::Session;
use crate::data::user::User;
use crate::db::session::{
    delete_by_token, delete_by_user, insert_session, select_session_by_token,
};
use crate::db::user::find_user_by_id;

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

pub fn authenticate(audience: &str, token: &str, is_continue: bool) -> Option<User> {
    if token.is_empty() {
        AuthenticationFailure::create(
            audience,
            "",
            AuthenticationMethod::Session,
            Reason::EmptySessionCookie,
        );
        return None;
    }

    let session = select_session_by_token(token);

    if session.is_none() {
        AuthenticationFailure::create(
            audience,
            "",
            AuthenticationMethod::Session,
            Reason::InvalidSessionCookie,
        );
        return None;
    }

    let session = session.unwrap();
    let user_id = session.user_id;
    let created_at = session.created_at;

    let user = find_user_by_id(&user_id);

    if user.is_none() {
        AuthenticationFailure::create(
            audience,
            "",
            AuthenticationMethod::Session,
            Reason::UserNotFound,
        );
        return None;
    }

    let user = user.unwrap();

    if !is_continue {
        Authentication::create(
            created_at,
            audience,
            &user.id,
            AuthenticationMethod::Session,
        );
    }

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
        let user = authenticate("aud.comame.dev", &session.token, false);

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
        let user = authenticate("aud.comame.dev", "dummy_session", false);

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

        let user = authenticate("aud.comame.dev", &session.token, false);

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

        let user = authenticate("aud.comame.dev", &session.token, false);

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

        let user_1 = authenticate("aud.comame.dev", &session_1.token, false);
        let user_2 = authenticate("aud.comame.dev", &session_2.token, false);

        assert_eq!(user_1.unwrap().id, user_id);
        assert_eq!(user_2.unwrap().id, user_id);
    }
}
