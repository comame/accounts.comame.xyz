use std::env;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use super::{authentication_flow_state, code_state};
use crate::data::authentication::{Authentication, AuthenticationMethod, LoginPrompt};
use crate::data::idtoken_issues::IdTokenIssue;
use crate::data::oidc_flow::authentication_flow_state::{
    AuthenticationFlowState, LoginRequirement, OidcFlow,
};
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::data::oidc_flow::authentication_response::{
    AuthenticationResponse, CodeFlowAuthenticationResponse, ImplicitFlowAuthenticationResponse,
};
use crate::data::oidc_flow::authenticationi_error_response::AuthenticationErrorResponse;
use crate::data::oidc_flow::code_state::CodeState;
use crate::data::oidc_flow::error_code::ErrorCode;
use crate::data::oidc_flow::id_token_claim::IdTokenClaim;
use crate::data::oidc_relying_party::RelyingParty;
use crate::data::rsa_keypair::RsaKeypair;
use crate::time::now;

#[derive(Debug)]
pub struct AuthenticationError {
    pub redirect_uri: Option<String>,
    pub flow: Option<OidcFlow>,
    pub response: AuthenticationErrorResponse,
}

/// Authentication Request を受け取って、ユーザ認証をする。
/// AuthenticationFlowState.login_requirement は認証要件を表す。
pub fn pre_authenticate(
    request: AuthenticationRequest,
) -> Result<AuthenticationFlowState, AuthenticationError> {
    let relying_party = RelyingParty::find(&request.client_id);
    if relying_party.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
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
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response,
        });
    }

    let flow = if request.scope.is("openid") {
        OidcFlow::Implicit
    } else if request.scope.is("openid code") {
        OidcFlow::Code
    } else {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidScope,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            flow: Some(OidcFlow::Code), // フローは未確定だが、クエリパラメータで返すのでこれでよい
            response,
        });
    };

    if (matches!(flow, OidcFlow::Implicit) && request.response_type != "id_token")
        || (matches!(flow, OidcFlow::Code) && request.response_type != "code")
    {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::UnsupportedResponseType,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            flow: Some(flow),
            response,
        });
    }

    if matches!(flow, OidcFlow::Implicit) && request.nonce.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: Some(request.redirect_uri),
            flow: Some(flow),
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
    if let Some(_max_age) = request.max_age {
        login_requirement = LoginRequirement::MaxAge;
    }

    let state = AuthenticationFlowState::new(
        &request.client_id,
        &request.redirect_uri,
        request.scope,
        request.state,
        request.nonce,
        request.max_age,
        login_requirement,
        flow,
    );
    authentication_flow_state::save_state(state.clone());

    Ok(state)
}

/// prompt=none のとき、interaction_required を返す
pub fn pronpt_none_fail_authentication(state_id: &str) -> AuthenticationError {
    let state = authentication_flow_state::get_state(state_id);
    if state.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InteractionRequired,
            state: None,
        };
        return AuthenticationError {
            redirect_uri: None,
            flow: None,
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
        flow: Some(state.flow),
        response,
    }
}

pub struct PostAuthenticationResponse {
    pub response: AuthenticationResponse,
    pub redirect_uri: String,
}

// ユーザー認証後、Authentication Response を行う
pub fn post_authentication(
    user_id: &str,
    state_id: &str,
    relying_party_id: &str,
    user_agent_id: &str,
    login_type: AuthenticationMethod,
) -> Result<PostAuthenticationResponse, AuthenticationError> {
    let state = authentication_flow_state::get_state(state_id);
    if state.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response,
        });
    }
    let state = state.unwrap();

    if state.relying_party_id.clone() != relying_party_id {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: state.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response,
        });
    }

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
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response,
        });
    }

    let latest_auth = Authentication::latest(user_id, user_agent_id);
    if latest_auth.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
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
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response,
        });
    }

    let claim = IdTokenClaim {
        iss: env::var("HOST").unwrap(),
        sub: user_id.to_string(),
        aud: state.relying_party_id.clone(),
        exp: now() + 5 * 60,
        iat: now(),
        auth_time: latest_auth.authenticated_at,
        nonce: state.nonce,
    };

    IdTokenIssue::log(&claim);

    let jwt_header = Header {
        alg: Algorithm::RS256,
        ..Default::default()
    };
    let jwt = encode(
        &jwt_header,
        &claim,
        &EncodingKey::from_rsa_pem(RsaKeypair::get().private.as_bytes()).unwrap(),
    )
    .unwrap();

    if let OidcFlow::Code = state.flow {
        let code = CodeState::new(
            &jwt,
            &state.relying_party_id,
            &state.scopes,
            &state.redirect_url,
        );
        code_state::save_state(&code);
        Ok(PostAuthenticationResponse {
            response: AuthenticationResponse::Code(CodeFlowAuthenticationResponse {
                state: state.state,
                code: code.code,
            }),
            redirect_uri: state.redirect_url,
        })
    } else {
        Ok(PostAuthenticationResponse {
            response: AuthenticationResponse::Implicit(ImplicitFlowAuthenticationResponse {
                state: state.state,
                id_token: jwt,
            }),
            redirect_uri: state.redirect_url,
        })
    }
}
