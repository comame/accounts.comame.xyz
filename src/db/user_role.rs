use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::user_role::UserRole;

pub struct UserRoleDb;

impl UserRoleDb {
    pub fn insert_ignore(user_role: &UserRole) {
        get_conn()
            .unwrap()
            .exec_drop(
                "INSERT IGNORE INTO user_role (user_id, role) VALUES (:user_id, :role)",
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

    pub fn findByUserId(user_id: &str) -> Vec<String> {
        get_conn()
            .unwrap()
            .exec_map(
                "SELECT role FROM user_role WHERE user_id=:user_id",
                params! {
                    "user_id" => user_id.to_string(),
                },
                |(role,)| role,
            )
            .unwrap()
    }
}
