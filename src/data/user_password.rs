#[derive(Debug, Clone)]
pub struct UserPassword {
    user_id: String,
    hashed_password: String,
}
