use mysql::params;
use mysql::prelude::*;

use super::mysql::{get_conn};
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
