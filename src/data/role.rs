use crate::db::role::RoleDb;

pub struct Role {
    pub name: String,
}

impl Role {
    pub fn new(name: &str) -> Self {
        RoleDb::insert(name);
        Self { name: name.into() }
    }

    pub fn get(name: &str) -> Option<Self> {
        RoleDb::get(name)
    }
}
