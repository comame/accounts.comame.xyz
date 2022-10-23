use mysql::prelude::*;
use mysql::{params, Error};

use super::mysql::get_conn;
use crate::data::user::User;

pub fn find_user_by_id(id: &str) -> Option<User> {
    let users = get_conn()
        .unwrap()
        .exec_map(
            "SELECT * FROM users WHERE id = :id",
            params! { "id" => id },
            |(id,)| User { id },
        )
        .unwrap();

    if users.is_empty() {
        None
    } else {
        Some(users.get(0).unwrap().clone())
    }
}

pub fn insert_user(user: &User) -> Result<(), Error> {
    get_conn().unwrap().exec_batch(
        "INSERT INTO users (id) VALUES (:id)",
        std::iter::once(params! { "id" => user.id.to_string() }),
    )
}

pub fn list_user() -> Vec<User> {
    get_conn()
        .unwrap()
        .query_map("SELECT * FROM users", |(id,)| User { id })
        .unwrap()
}

pub fn delete_user(user_id: &str) -> Result<(), ()> {
    let result = get_conn().unwrap().exec_drop(
        "DELETE FROM users WHERE id=:id",
        params! { "id" => user_id.to_string() },
    );

    if result.is_err() {
        Err(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::_test_init::init_mysql;
    use super::*;

    fn generate_user(id: &str) -> User {
        User { id: id.to_string() }
    }

    #[test]
    fn create_user() {
        init_mysql();
        let result = insert_user(&generate_user("db-user-create-user"));
        assert!(result.is_ok());
    }

    #[test]
    fn can_find_user() {
        init_mysql();
        let user_id = "db-user-can-find-user";
        let user = generate_user(user_id);
        insert_user(&user).unwrap();
        let result = find_user_by_id(user_id);
        assert_eq!(user, result.unwrap());
    }

    #[test]
    fn fail_find_user() {
        init_mysql();
        let user_id = "db-user-fail-find-user";
        let user = generate_user(user_id);
        insert_user(&user).unwrap();
        let result = find_user_by_id("bar");
        assert!(result.is_none());
    }
}
