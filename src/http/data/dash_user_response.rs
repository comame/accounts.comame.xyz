use serde::Serialize;

use crate::{dash::user::UserWithPassword, data::idtoken_issues::IdTokenIssue};

#[derive(Serialize)]
pub struct ListUserRespnse {
    pub values: Vec<UserWithPassword>,
}

#[derive(Serialize)]
pub struct IdTokenIssueResponse {
    pub values: Vec<IdTokenIssue>,
}
