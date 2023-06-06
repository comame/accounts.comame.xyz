use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::role::Role;

pub struct RoleDb;

impl RoleDb {
    pub fn insert(name: &str) {
        get_conn()
            .unwrap()
            .exec_drop(
                "
            INSERT INTO `role` (`name`) VALUES (:name)
        ",
                params! {
                    "name" => name.to_string(),
                },
            )
            .unwrap();
    }

    pub fn get(name: &str) -> Option<Role> {
        let result: (String,) = get_conn()
            .unwrap()
            .exec_first(
                "
        SELECT name FROM role WHERE name = :name
        ",
                params! {
                    "name" => name.to_string()
                },
            )
            .unwrap()?;
        Some(Role { name: result.0 })
    }
}
