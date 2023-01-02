use crate::data::access_token::AccessToken;
use crate::data::oidc_flow::userinfo::UserInfo;

#[derive(Debug)]
pub enum ErrorReason {
    InvalidToken,
    InsufficientScope,
}

pub fn userinfo(access_token: &str) -> Result<UserInfo, ErrorReason> {
    let access_token = AccessToken::get(access_token);
    if access_token.is_none() {
        return Err(ErrorReason::InvalidToken);
    }
    let access_token = access_token.unwrap();

    // TODO: Google アカウントなら Google から取得する

    let userinfo =
        UserInfo::get(&access_token.sub).unwrap_or_else(|| UserInfo::empty(&access_token.sub));

    let scopes = access_token.scopes;

    let mut response = UserInfo::empty(&access_token.sub);

    if !scopes.has("email") && !scopes.has("profile") {
        return Err(ErrorReason::InsufficientScope);
    }

    if scopes.has("email") {
        response.email = userinfo.email;
        response.email_verified = userinfo.email_verified;
    }

    if scopes.has("profile") {
        response.name = userinfo.name;
        response.preferred_username = userinfo.preferred_username;
        response.profile = userinfo.profile;
        response.picture = userinfo.picture;
    }

    Ok(response)
}
