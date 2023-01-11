use crate::db::user_role::UserRoleDb;

use super::role::Role;

pub struct UserRole {
    pub user_id: String,
    pub role: String,
}

impl UserRole {
    pub fn new(user_id: &str, role: &str) -> Result<Self, ()> {
        let role_exists = Role::get(role).is_some();
        if !role_exists {
            return Err(());
        }
        let v = Self {
            user_id: user_id.into(),
            role: role.into(),
        };
        UserRoleDb::new(&v);
        Ok(v)
    }

    pub fn exists(&self) -> bool {
        UserRoleDb::exists(self)
    }
}
