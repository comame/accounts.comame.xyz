use serde::Serialize;

use crate::auth::password::set_password;
use crate::data::user::User;

#[derive(Serialize)]
pub struct UserWithPassword {
    user_id: String,
    has_password: bool,
}

pub fn list() -> Vec<UserWithPassword> {
    let users = User::all();
    users
        .iter()
        .map(|user| UserWithPassword {
            user_id: user.id.to_string(),
            has_password: user.has_password(),
        })
        .collect()
}

pub fn create(user_id: &str) -> Result<User, ()> {
    User::new(user_id)
}

pub fn delete(user_id: &str) -> Result<(), ()> {
    User::delete(user_id)
}

pub fn insert_password(user_id: &str, password: &str) -> Result<(), ()> {
    let user = User::find(user_id);
    if user.is_none() {
        return Err(());
    }

    set_password(user_id, password);
    Ok(())
}

pub fn remove_password(user_id: &str) {
    let user = User::find(user_id);
    if user.is_none() {
        return;
    }

    user.unwrap().remove_password();
}
