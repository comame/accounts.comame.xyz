use super::authenticate_client::authenticate_client;
use crate::auth::password::calculate_password_hash;
use crate::data::external::session::ExternalSession;
use crate::data::oidc_relying_party::RelyingParty;

pub fn create_session(client_id: &str, client_secret: &str, user_id: &str) -> Result<String, ()> {
    let _authenticated = authenticate_client(client_id, client_secret)?;
    let token = ExternalSession::create(client_id, user_id)?;

    Ok(token.token)
}

/// Returns user_id
pub fn inspect_token(client_id: &str, token: &str) -> Option<String> {
    let session = ExternalSession::get(client_id, token)?;
    Some(session.user_id)
}
