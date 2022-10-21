use super::code_state::get_state;
use crate::auth::password::calculate_password_hash;
use crate::crypto::rand::random_str;
use crate::data::oidc_flow::code_request::CodeRequest;
use crate::data::oidc_flow::code_response::CodeResponse;
use crate::data::oidc_relying_party::RelyingParty;

pub fn code_request(req: CodeRequest) -> Result<CodeResponse, ()> {
    let relying_party = RelyingParty::find(&req.client_id);
    if relying_party.is_none() {
        dbg!("invalid");
        return Err(());
    }
    let relying_party = relying_party.unwrap();

    if req.client_secret.is_none() {
        dbg!("invalid");
        return Err(());
    }

    if calculate_password_hash(&req.client_secret.unwrap(), &req.client_id)
        != relying_party.hashed_client_secret
    {
        dbg!("invalid");
        return Err(());
    }

    let saved_state = get_state(&req.code);
    if saved_state.is_none() {
        dbg!("invalid");
        return Err(());
    }
    let saved_state = saved_state.unwrap();

    if req.client_id != saved_state.client_id {
        dbg!("invalid");
        return Err(());
    }

    if req.redirect_uri != saved_state.redirect_uri {
        dbg!("invalid");
        return Err(());
    }

    if req.grant_type != "authorization_code" {
        dbg!("invalid");
        return Err(());
    }

    Ok(CodeResponse {
        access_token: random_str(16),
        token_type: "bearer".to_string(),
        id_token: saved_state.id_token,
        scope: saved_state.scope,
    })
}
