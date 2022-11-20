use crate::data::{
    access_token::{self, AccessToken},
    oidc_flow::userinfo_reponse::UserInfoResponse,
};

#[derive(Debug)]
pub enum ErrorReason {
    InvalidToken,
    InsufficientScope,
}

pub fn userinfo(access_token: &str) -> Result<UserInfoResponse, ErrorReason> {
    let access_token = AccessToken::get(&access_token);
    if access_token.is_none() {
        return Err(ErrorReason::InvalidToken);
    }
    let access_token = access_token.unwrap();

    let mut response = UserInfoResponse {
        sub: access_token.sub,
        email: None,
        email_verified: None,
        name: None,
        preferred_username: None,
        profile: None,
        picture: None,
    };

    let scopes = access_token.scopes;

    if !scopes.has("email") && !scopes.has("profile") {
        return Err(ErrorReason::InsufficientScope);
    }

    if scopes.has("email") {
        // もしあれば返す
    }

    if scopes.has("profile") {
        // もしあれば返す
    }

    Ok(response)
}
