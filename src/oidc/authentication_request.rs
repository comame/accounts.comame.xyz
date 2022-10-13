use super::authentication_flow_state::save_state;
use crate::data::authentication::LoginPrompt;
use crate::data::oidc_flow::authentication_flow_state::{
    AuthenticationFlowState, LoginRequirement,
};
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::data::oidc_flow::authenticationi_error_response::AuthenticationErrorResponse;
use crate::data::oidc_flow::error_code::ErrorCode;
use crate::data::oidc_relying_party::RelyingParty;

#[derive(Debug)]
pub struct PreAuthenticationError {
    pub redirect_uri: Option<String>,
    pub response: AuthenticationErrorResponse,
}

pub fn pre_authenticate(
    request: AuthenticationRequest,
) -> Result<AuthenticationFlowState, PreAuthenticationError> {
    let relying_party = RelyingParty::find(&request.client_id);
    if relying_party.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        return Err(PreAuthenticationError {
            redirect_uri: None,
            response,
        });
    }
    let relying_party = relying_party.unwrap();

    let mut is_redirect_uri_match = false;
    for ref url in relying_party.redirect_uris {
        if url == &request.redirect_uri {
            is_redirect_uri_match = true;
        }
    }
    if !is_redirect_uri_match {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        return Err(PreAuthenticationError {
            redirect_uri: None,
            response,
        });
    }

    if request.scope != "openid" {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidScope,
            state: request.state,
        };
        return Err(PreAuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            response,
        });
    }

    if request.response_type != "id_token" {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::UnsupportedResponseType,
            state: request.state,
        };
        dbg!(&response);
        return Err(PreAuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            response,
        });
    }

    if request.nonce.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        return Err(PreAuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            response,
        });
    }

    if request.id_token_hint.is_some() {
        todo!();
    }

    let mut login_requirement = LoginRequirement::Any;
    if request.max_age.is_some() {
        login_requirement = LoginRequirement::MaxAge;
    }
    if request.prompt.is_none() {
        login_requirement = LoginRequirement::Any;
    }
    if let Some(prompt) = request.prompt {
        match prompt {
            LoginPrompt::Consent => {
                login_requirement = LoginRequirement::Consent;
            }
            LoginPrompt::Login => {
                login_requirement = LoginRequirement::ReAuthenticate;
            }
            LoginPrompt::None => {
                login_requirement = LoginRequirement::None;
            }
            LoginPrompt::SelectAccount => {
                login_requirement = LoginRequirement::Consent;
            }
        }
    }

    let state = AuthenticationFlowState::new(
        &request.client_id,
        &request.redirect_uri,
        request.state,
        request.nonce,
        request.max_age,
        login_requirement,
    );
    save_state(state.clone());

    Ok(state)
}
