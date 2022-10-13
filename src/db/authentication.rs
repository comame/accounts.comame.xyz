use mysql::params;
use mysql::prelude::Queryable;

use super::mysql::get_conn;
use crate::data::authentication::Authentication;
use crate::db::mysql::mysqldate_to_unixtime;
use crate::time::unixtime_to_datetime;

pub fn insert_authentication(auth: &Authentication) {
    get_conn()
        .unwrap()
        .exec_batch(
            "INSERT INTO authentications values (:auth_at, :cr_at, :aud, :sub, :met)",
            std::iter::once(params! {
                "auth_at" => unixtime_to_datetime(auth.authenticated_at),
                "cr_at" => unixtime_to_datetime(auth.created_at),
                "aud" => auth.audience.clone(),
                "sub" => auth.subject.clone(),
                "met" => auth.method.to_string(),
            }),
        )
        .unwrap();
}

pub fn find_latest_authentication_by_user(_user_id: &str) -> Option<Authentication> {
    let result = get_conn().unwrap().query_map(
        "SELECT * FROM authentications WHERE method NOT LIKE \"session\" ORDER BY created_at DESC LIMIT 1",
        |tuple: (mysql::Value, mysql::Value, String, String, String, String)| Authentication {
            authenticated_at: mysqldate_to_unixtime(tuple.0),
            created_at: mysqldate_to_unixtime(tuple.1),
            audience: tuple.2,
            subject: tuple.3,
            method: tuple.4.as_str().try_into().unwrap(),
        }).unwrap();

    result.first().cloned()
}
