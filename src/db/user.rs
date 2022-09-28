use crate::data::user::User;
use super::mysql::get_conn;
use mysql::{ params, Error };
use mysql::prelude::*;

pub fn find_user_by_id(id: &str) -> Option<User> {
    let users = get_conn().unwrap().exec_map(
        "SELECT * FROM users WHERE id = :id",
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::_test_init::init;

    fn generate_user(id: &str) -> User {
        User { id: id.to_string() }
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_create_user() {
        init();
        let result = insert_user(&generate_user("foo"));
        assert!(result.is_ok());
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_can_find_user() {
        init();
        let user = generate_user("foo");
        insert_user(&user).unwrap();
        let result = find_user_by_id("foo");
        assert_eq!(user, result.unwrap());
    }

    #[test]
    #[ignore = "Single thread only"]
    fn single_thread_fail_find_user() {
        init();
        let user = generate_user("foo");
        insert_user(&user).unwrap();
        let result = find_user_by_id("bar");
        assert!(result.is_none());
    }
}
