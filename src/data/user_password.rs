#[derive(Debug, Clone)]
pub struct UserPassword {
    pub user_id: String,
    pub hashed_password: String,
}
