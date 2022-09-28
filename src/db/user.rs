use crate::data::user::User;
use super::mysql::get_conn;
use mysql::{ params, Error };
use mysql::prelude::*;

pub fn find_user_by_id(id: &str) -> Option<User> {
    let users = get_conn().unwrap().exec_map(
        "SELECT id FROM users WHERE id = :id",
        params! { "id" => id },
        |(id,)| User { id }
    ).unwrap();

    if users.len() > 0 {
        Some(users.get(0).unwrap().clone())
    } else {
        None
    }
}


pub fn insert_user(user: &User) -> Result<(), Error> {
    let result = get_conn().unwrap().exec_batch(
        "INSERT INTO users (id) VALUES (:id)",
        std::iter::once(params! { "id" => user.id.to_string() })
    );

    if let Err(error) = result {
        return Err(error);
    } else {
        return Ok(());
    }
}
