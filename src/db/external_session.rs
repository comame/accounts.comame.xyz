use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::external::session::ExternalSession;
use crate::db::mysql::mysqldate_to_unixtime;
use crate::time::unixtime_to_datetime;

pub fn insert_session(session: &ExternalSession) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO external_sessions VALUES (:cid, :uid, :token, :cat",
            params! {
                "cid" => session.client_id.clone(),
                "uid" => session.user_id.clone(),
                "token" => session.token.clone(),
                "created_at" => unixtime_to_datetime(session.created_at),
            },
        )
        .unwrap();
}

pub fn get_session(client_id: &str, token: &str) -> Option<ExternalSession> {
    let result = get_conn()
        .unwrap()
        .exec_map(
            "SELECT * FROM external_sessions WHERE client_id=:cid AND token=:token",
            params! {
                "cid" => client_id.to_string(),
                "token" => token.to_string(),
            },
            |(client_id, user_id, token, created_at)| ExternalSession {
                client_id,
                user_id,
                token,
                created_at: mysqldate_to_unixtime(created_at),
            },
        )
        .unwrap();
    result.first().cloned()
}
