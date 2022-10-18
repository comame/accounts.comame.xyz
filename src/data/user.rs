use crate::db::user::find_user_by_id;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
}

impl User {
    pub fn find(user_id: &str) -> Option<User> {
        find_user_by_id(user_id)
    }
}
