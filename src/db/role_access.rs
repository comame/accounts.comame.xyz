use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;

pub struct RoleAccessDb;

impl RoleAccessDb {
    pub fn new(role: &str, relying_party_id: &str) {
        get_conn().unwrap().exec_drop(
            "INSERT INTO role_access (role, relying_party_id) VALUES (:role, :relying_party_id)",
            params! {
                "role" => role.to_string(),
                "relying_party_id" => relying_party_id.to_string(),
            }
        ).unwrap()
    }

    pub fn is_accessible(user_id: &str, relying_party_id: &str) -> bool {
        let result: (usize,) = get_conn().unwrap().exec_first(
            "SELECT COUNT(*) FROM user_role INNER JOIN role_access ON user_role.role = role_access.role WHERE user_id = :user_id AND relying_party_id = :relying_party_id",
            params! {
                "user_id" => user_id.to_string(),
                "relying_party_id" => relying_party_id.to_string(),
            }
        ).unwrap().unwrap();
        result.0 != 0
    }
}
