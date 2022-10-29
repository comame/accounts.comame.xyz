use mysql::{params, prelude::*};

use crate::{data::idtoken_issues::IdTokenIssues, time::unixtime_to_datetime};

use super::mysql::get_conn;

pub fn insert(claim: &IdTokenIssues) {
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
