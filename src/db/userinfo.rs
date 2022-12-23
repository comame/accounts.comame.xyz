use mysql::params;
use mysql::prelude::*;
use serde_json::from_str;

use super::mysql::get_conn;
use crate::data::oidc_flow::userinfo::UserInfo;

pub fn get_userinfo(sub: &str) -> Option<UserInfo> {
    let result: Vec<(String, String)> = get_conn()
        .unwrap()
        .exec_map(
            "SELECT sub,value FROM userinfo WHERE sub=:sub",
            params! {
                "sub" => sub.to_string()
            },
            |(sub, value)| (sub, value),
        )
        .unwrap();
    let first = result.first()?.clone();

    let result = from_str::<UserInfo>(&first.1);

    if result.is_err() {
        None
    } else {
        Some(result.unwrap())
    }
}
