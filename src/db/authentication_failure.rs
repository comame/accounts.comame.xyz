use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::authentication_failure::AuthenticationFailure;
use crate::time::unixtime_to_datetime;

pub fn insert_fail(fail: &AuthenticationFailure) {
    get_conn()
        .unwrap()
        .exec_drop(
            "INSERT INTO authentication_failures VALUES (:at, :sub, :met, :rea, FALSE, :addr)",
            params! {
                "at" => unixtime_to_datetime(fail.tried_at),
                "sub" => fail.subject_input.clone(),
                "met" => fail.method.to_string(),
                "rea" => fail.reason.to_string(),
                "addr" => fail.remote_addr.to_string(),
            },
        )
        .unwrap();
}

pub fn count_recent_log(user_id: &str) -> u8 {
    let result: Vec<u8> = get_conn().unwrap().exec_map(
        "SELECT COUNT(*) FROM (SELECT subject, tried_at FROM authentication_failures  WHERE subject = :user AND TIMESTAMPDIFF(MINUTE, tried_at, CURRENT_TIME) < :expire AND clean = FALSE ORDER BY tried_at DESC limit 10) a",
        params! { "user" => user_id.to_string(), "expire" => 24 * 60 },
        |(count,)| count
    ).unwrap();
    *result.first().unwrap()
}
