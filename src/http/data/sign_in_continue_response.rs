use serde::Serialize;

#[derive(Serialize)]
pub struct SigninContinueSuccessResponse {
    pub location: String,
}
