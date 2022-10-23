use serde::Serialize;

use crate::dash::user::UserWithPassword;

#[derive(Serialize)]
pub struct ListUserRespnse {
    pub values: Vec<UserWithPassword>,
}
