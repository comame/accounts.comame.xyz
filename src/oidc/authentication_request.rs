use jsonwebtoken::{encode, EncodingKey, Header};

use super::authentication_flow_state::{get_state, save_state};
use crate::data::authentication::{Authentication, AuthenticationMethod, LoginPrompt};
use crate::data::oidc_flow::authentication_flow_state::{
    AuthenticationFlowState, LoginRequirement,
};
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::data::oidc_flow::authentication_response::AuthenticationResponse;
use crate::data::oidc_flow::authenticationi_error_response::AuthenticationErrorResponse;
use crate::data::oidc_flow::error_code::ErrorCode;
use crate::data::oidc_flow::id_token_claim::IdTokenClaim;
use crate::data::oidc_relying_party::RelyingParty;
use crate::time::now;

#[derive(Debug)]
pub struct AuthenticationError {
    pub redirect_uri: Option<String>,
    pub response: AuthenticationErrorResponse,
}

pub fn pre_authenticate(
    request: AuthenticationRequest,
) -> Result<AuthenticationFlowState, AuthenticationError> {
    let relying_party = RelyingParty::find(&request.client_id);
    if relying_party.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        return Err(AuthenticationError {
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
        return Err(AuthenticationError {
            redirect_uri: None,
            response,
        });
    }

    if request.scope != "openid" {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidScope,
            state: request.state,
        };
        return Err(AuthenticationError {
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
        return Err(AuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            response,
        });
    }

    if request.nonce.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        return Err(AuthenticationError {
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
    if let Some(max_age) = request.max_age {
        login_requirement = LoginRequirement::MaxAge;
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

pub fn pronpt_none_fail_authentication(state_id: &str) -> AuthenticationError {
    let state = get_state(state_id);
    if state.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InteractionRequired,
            state: None,
        };
        return AuthenticationError {
            redirect_uri: None,
            response,
        };
    }
    let state = state.unwrap();

    let response = AuthenticationErrorResponse {
        error: ErrorCode::InteractionRequired,
        state: state.state,
    };
    AuthenticationError {
        redirect_uri: Some(state.redirect_url),
        response,
    }
}

pub struct PostAuthenticationResponse {
    pub response: AuthenticationResponse,
    pub redirect_uri: String,
}

pub fn post_authentication(
    user_id: &str,
    state_id: &str,
    login_type: AuthenticationMethod,
) -> Result<PostAuthenticationResponse, AuthenticationError> {
    let state = get_state(state_id);
    if state.is_none() {
        dbg!("no state");
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        return Err(AuthenticationError {
            redirect_uri: None,
            response,
        });
    }
    let state = state.unwrap();

    let auth_level_ok = match state.login_requirement {
        LoginRequirement::Consent => login_type != AuthenticationMethod::Session,
        LoginRequirement::ReAuthenticate => {
            !(login_type == AuthenticationMethod::Session
                || login_type == AuthenticationMethod::Consent)
        }

        LoginRequirement::None => login_type == AuthenticationMethod::Session,
        LoginRequirement::Any => true,
        LoginRequirement::MaxAge => true,
    };
    if !auth_level_ok {
        dbg!("auth level invalid");
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        return Err(AuthenticationError {
            redirect_uri: None,
            response,
        });
    }

    let latest_auth = Authentication::latest(user_id);
    if latest_auth.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        return Err(AuthenticationError {
            redirect_uri: None,
            response,
        });
    }
    let latest_auth = latest_auth.unwrap();

    if state.login_requirement == LoginRequirement::MaxAge
        && now() - latest_auth.authenticated_at > state.max_age.unwrap()
    {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        return Err(AuthenticationError {
            redirect_uri: None,
            response,
        });
    }

    let claim = IdTokenClaim {
        iss: "https://id.comame.xyz".to_string(),
        sub: user_id.to_string(),
        aud: state.relying_party_id,
        exp: now() + 5 * 60,
        iat: now(),
        auth_time: latest_auth.authenticated_at,
        nonce: state.nonce,
    };

    let jwt = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    Ok(PostAuthenticationResponse {
        response: AuthenticationResponse {
            id_token: jwt,
            state: state.state,
        },
        redirect_uri: state.redirect_url,
    })
}