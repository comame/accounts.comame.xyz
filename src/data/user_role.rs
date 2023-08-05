use super::role::Role;
use crate::db::user_role::UserRoleDb;

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
        UserRoleDb::insert_ignore(&v);
        Ok(v)
    }

    pub fn exists(&self) -> bool {
        UserRoleDb::exists(self)
    }

    pub fn list(user_id: &str) -> Vec<UserRole> {
        let roles = UserRoleDb::findByUserId(user_id);
        roles
            .iter()
            .map(|role| UserRole {
                user_id: user_id.to_string(),
                role: role.to_string(),
            })
            .collect()
    }
}
