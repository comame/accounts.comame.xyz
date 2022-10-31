use mysql::{params, prelude::*};

use crate::{data::idtoken_issues::IdTokenIssue, time::unixtime_to_datetime};

use super::mysql::{get_conn, mysqldate_to_unixtime};

pub fn insert(claim: &IdTokenIssue) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO idtoken_issues VALUES (:sub, :aud, :iat)",
            params! {
                "sub" => claim.sub.clone(),
                "aud" => claim.aud.clone(),
                "iat" => unixtime_to_datetime(claim.iat),
            },
        )
        .unwrap()
}

pub fn list_by_sub(subject: &str) -> Vec<IdTokenIssue> {
    let result = get_conn()
        .unwrap()
        .exec_map(
            "SELECT * FROM idtoken_issues WHERE sub=:sub ORDER BY iat DESC",
            params! {
                "sub" => subject.to_string()
            },
            |(sub, aud, iat)| IdTokenIssue {
                sub,
                aud,
                iat: mysqldate_to_unixtime(iat),
            },
        )
        .unwrap();
    result
}
