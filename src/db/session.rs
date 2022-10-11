use crate::time::unixtime_to_datetime;
use crate::{data::session::Session, db::mysql::get_conn};
use mysql::params;
use mysql::prelude::*;

use super::mysql::mysqldate_to_unixtime;

pub fn insert_session(session: &Session) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO sessions (user_id, token, created_at) VALUES (:user, :token, :at)",
            params! {
                "user" => session.user_id.clone(),
                "token" => session.token.clone(),
                "at" => unixtime_to_datetime(session.created_at),
            },
        )
        .unwrap();
}

pub fn select_session_by_token(token: &str) -> Option<Session> {
    let sessions: Vec<Session> = get_conn()
        .unwrap()
        .exec_map(
            "SELECT user_id, token, created_at FROM sessions WHERE token = :token",
            params! {
                "token" => token,
            },
            |(user_id, token, created_at): (String, String, mysql::Value)| Session {
                user_id,
                token,
                created_at: mysqldate_to_unixtime(created_at),
            },
        )
        .unwrap();
    let session = sessions.get(0)?;
    Some(session.clone())
}

pub fn delete_by_token(token: &str) {
    get_conn()
        .unwrap()
        .exec_drop(
            "DELETE FROM sessions WHERE token = :token",
            params! {
                "token" => token
            },
        )
        .unwrap();
}

pub fn delete_by_user(user_id: &str) {
    get_conn()
        .unwrap()
        .exec_drop(
            "DELETE FROM sessions WHERE user_id = :user_id",
            params! {
                "user_id" => user_id
            },
        )
        .unwrap()
}
