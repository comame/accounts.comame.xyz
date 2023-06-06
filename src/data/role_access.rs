use crate::db::role_access::RoleAccessDb;

pub struct RoleAccess {
    pub role: String,
    pub relying_party_id: String,
}

impl RoleAccess {
    pub fn new(role: &str, relying_party_id: &str) -> Self {
        RoleAccessDb::create(role, relying_party_id);
        Self {
            role: role.into(),
            relying_party_id: relying_party_id.into(),
        }
    }

    pub fn is_accessible(user_id: &str, relying_party_id: &str) -> bool {
        RoleAccessDb::is_accessible(user_id, relying_party_id)
    }
}
