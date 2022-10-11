use mysql::{params, prelude::Queryable};

use crate::{data::authentication_failure::AuthenticationFailure, time::unixtime_to_datetime};

use super::mysql::get_conn;

pub fn insert_authentication_failure(failure: &AuthenticationFailure) {
    get_conn()
        .unwrap()
        .exec_batch(
            "INSERT INTO authentication_failures values (:at, :aud, :sub, :met, :reason)",
            std::iter::once(params! {
                "at" => unixtime_to_datetime(failure.tried_at),
                "aud" => failure.audience.clone(),
                "sub" => failure.subject.clone(),
                "met" => failure.method.to_string(),
                "reason" => failure.reason.to_string(),
            }),
        )
        .unwrap();
}
