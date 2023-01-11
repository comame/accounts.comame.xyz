use mysql::{params, prelude::*};

use crate::data::user_role::UserRole;

use super::mysql::get_conn;

pub struct UserRoleDb;

impl UserRoleDb {
    pub fn new(user_role: &UserRole) {
        get_conn()
            .unwrap()
            .exec_drop(
                "INSERT INTO user_role (user_id, role) VALUES (:user_id, :role)",
                params! {
                    "user_id" => user_role.user_id.to_string(),
                    "role" => user_role.role.to_string(),
                },
            )
            .unwrap();
    }

    pub fn exists(user_role: &UserRole) -> bool {
        let result: (usize,) = get_conn()
            .unwrap()
            .exec_first(
                "SELECT COUNT(*) FROM user_role WHERE user_id = :user_id AND role = :role",
                params! {
                    "user_id" => user_role.user_id.to_string(),
                    "role" => user_role.role.to_string(),
                },
            )
            .unwrap()
            .unwrap();
        result.0 != 0
    }
}
