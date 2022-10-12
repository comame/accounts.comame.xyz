use super::error_code::ErrorCode;

#[derive(Debug)]
pub struct AuthenticationErrorResponse {
    pub error: ErrorCode,
    pub state: Option<String>,
}
