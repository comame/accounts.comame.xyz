use crate::auth::session::revoke_session_by_user_id;
use crate::db::user::{delete_user, find_user_by_id, insert_user, list_user};
use crate::db::user_password::{password_exists, remove_password};

use super::user_role::UserRole;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn all() -> Vec<Self> {
        list_user()
    }

    pub fn find(user_id: &str) -> Option<User> {
        find_user_by_id(user_id)
    }

    pub fn delete(user_id: &str) -> Result<(), ()> {
        delete_user(user_id)
    }

    pub fn new(user_id: &str) -> Result<Self, ()> {
        let user = User {
            id: user_id.to_string(),
        };
        let result = insert_user(&user);

        if result.is_err() {
            Err(())
        } else {
            UserRole::new(user_id, "everyone").unwrap();
            Ok(user)
        }
    }

    pub fn has_password(&self) -> bool {
        password_exists(&self.id)
    }

    pub fn remove_password(&self) {
        remove_password(&self.id);
        revoke_session_by_user_id(&self.id);
    }

    pub fn lock(&self) {
        self.remove_password();
        revoke_session_by_user_id(&self.id);
    }
}
