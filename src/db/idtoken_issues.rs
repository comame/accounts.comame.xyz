use mysql::params;
use mysql::prelude::*;

use super::mysql::{get_conn, mysqldate_to_unixtime};
use crate::data::idtoken_issues::IdTokenIssue;
use crate::time::unixtime_to_datetime;

pub fn insert(claim: &IdTokenIssue) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO idtoken_issues VALUES (:sub, :aud, :iat, :addr)",
            params! {
                "sub" => claim.sub.clone(),
                "aud" => claim.aud.clone(),
                "iat" => unixtime_to_datetime(claim.iat),
                "addr" => claim.remote_addr.to_string(),
            },
        )
        .unwrap()
}

pub fn list_by_sub(subject: &str) -> Vec<IdTokenIssue> {
    get_conn()
        .unwrap()
        .exec_map(
            "SELECT * FROM idtoken_issues WHERE sub=:sub ORDER BY iat DESC",
            params! {
                "sub" => subject.to_string()
            },
            |(sub, aud, iat, remote_addr)| IdTokenIssue {
                sub,
                aud,
                iat: mysqldate_to_unixtime(iat),
                remote_addr,
            },
        )
        .unwrap()
}
