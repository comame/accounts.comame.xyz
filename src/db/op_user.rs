use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::op_user::OpUser;
use crate::data::openid_provider::OpenIDProvider;

pub struct OpUserDb;

impl OpUserDb {
    pub fn get(op_user_id: &str, op: OpenIDProvider) -> Option<OpUser> {
        let result: (String, String, String) = get_conn().unwrap().exec_first(
            "SELECT user_id, op_user_id, op FROM op_user WHERE op_user_id=:op_user_id AND op=:op",
            params! {
                "op_user_id" => op_user_id.to_string(),
                "op" => op.to_string()
            },
        ).unwrap()?;
        let op_user = OpUser {
            user_id: result.0,
            op_user_id: result.1,
            op: OpenIDProvider::parse(&result.2).unwrap(),
        };
        Some(op_user)
    }
}
