use super::user_role::UserRole;
use crate::auth::session::revoke_session_by_user_id;
use crate::db::user::{find_user_by_id, insert_user};
use crate::db::user_password::remove_password;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn find(user_id: &str) -> Option<User> {
        find_user_by_id(user_id)
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

    pub fn remove_password(&self) {
        remove_password(&self.id);
        revoke_session_by_user_id(&self.id);
    }

    pub fn lock(&self) {
        self.remove_password();
        revoke_session_by_user_id(&self.id);
    }
}
