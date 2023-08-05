use std::env;

use http::response::Response;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use url::Url;

use super::code_state;
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
use crate::data::openid_provider::OpenIDProvider;
use crate::data::role::Role;
use crate::data::rsa_keypair::RsaKeypair;
use crate::data::user::User;
use crate::data::user_role::UserRole;
use crate::time::now;

#[derive(Debug)]
pub struct AuthenticationError {
    pub redirect_uri: Option<String>,
    pub flow: Option<OidcFlow>,
    pub response: AuthenticationErrorResponse,
    pub client_id: String,
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
            client_id: String::from(""),
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
            client_id: relying_party.client_id,
            redirect_uri: None,
            flow: None,
            response,
        });
    }

    if !request.scope.within("openid profile email") {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidScope,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            client_id: relying_party.client_id,
            redirect_uri: Some(request.redirect_uri),
            flow: Some(OidcFlow::Code), // フローは未確定だが、クエリパラメータで返すのでこれでよい
            response,
        });
    }

    let flow = if request.response_type == "code" {
        OidcFlow::Code
    } else if request.response_type == "id_token" {
        OidcFlow::Implicit
    } else {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::UnsupportedResponseType,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            client_id: relying_party.client_id,
            redirect_uri: Some(request.redirect_uri),
            flow: Some(OidcFlow::Code), // フローは未確定だが、クエリパラメータで返すのでこれでよい
            response,
        });
    };

    if matches!(flow, OidcFlow::Implicit) && request.nonce.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: request.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            client_id: relying_party.client_id,
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

    Ok(state)
}

#[derive(Debug)]
pub struct PostAuthenticationResponse {
    pub response: AuthenticationResponse,
    pub redirect_uri: String,
    pub client_id: String,
}

// ユーザー認証後、Authentication Response を行う
pub fn post_authentication(
    user_id: &str,
    state_id: &str,
    relying_party_id: &str,
    user_agent_id: &str,
    login_type: AuthenticationMethod,
    remote_addr: &str,
) -> Result<PostAuthenticationResponse, AuthenticationError> {
    let state = AuthenticationFlowState::get_consume(state_id);
    if state.is_none() {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: None,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            client_id: relying_party_id.to_string(),
            redirect_uri: None,
            flow: None,
            response,
        });
    }
    let state = state.unwrap();

    if state.relying_party_id != relying_party_id {
        let response = AuthenticationErrorResponse {
            error: ErrorCode::InvalidRequest,
            state: state.state,
        };
        dbg!("invalid");
        return Err(AuthenticationError {
            client_id: relying_party_id.to_string(),
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
            client_id: relying_party_id.to_string(),
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
            client_id: relying_party_id.to_string(),
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
            client_id: relying_party_id.to_string(),
            redirect_uri: None,
            flow: None,
            response,
        });
    }

    let roles = UserRole::list(user_id);
    let roles = roles.iter().map(|r| r.role.clone()).collect();

    let mut claim = IdTokenClaim {
        iss: env::var("HOST").unwrap(),
        sub: user_id.into(),
        aud: state.relying_party_id.clone(),
        exp: now() + 5 * 60,
        iat: now(),
        auth_time: Some(latest_auth.authenticated_at),
        nonce: state.nonce,
        email: None,
        email_verified: None,
        name: None,
        preferred_username: None,
        profile: None,
        picture: None,
        roles: Some(roles),
    };

    if state.scopes.has("email") {
        // set email scope if present
    }

    if state.scopes.has("profile") {
        // set profiles if present
        claim.name = Some(user_id.to_string());
    }

    IdTokenIssue::log(&claim, remote_addr);

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

    let federated_rp = if login_type == AuthenticationMethod::Google {
        Some(OpenIDProvider::Google)
    } else {
        None
    };
    if let OidcFlow::Code = state.flow {
        let code = CodeState::new(
            &jwt,
            &state.relying_party_id,
            &state.scopes,
            &state.redirect_url,
            user_id,
            federated_rp,
        );
        code_state::save_state(&code);
        Ok(PostAuthenticationResponse {
            client_id: relying_party_id.to_string(),
            response: AuthenticationResponse::Code(CodeFlowAuthenticationResponse {
                state: state.state,
                code: code.code,
            }),
            redirect_uri: state.redirect_url,
        })
    } else {
        Ok(PostAuthenticationResponse {
            client_id: relying_party_id.to_string(),
            response: AuthenticationResponse::Implicit(ImplicitFlowAuthenticationResponse {
                state: state.state,
                id_token: jwt,
            }),
            redirect_uri: state.redirect_url,
        })
    }
}

fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.status = 400;
    res
}

fn response_redirect(url: &str) -> Response {
    let mut res = Response::new();
    res.body = Some(format!(r#"{{ "location": "{url}" }}"#));
    res
}

pub fn response_post_authentication(
    result: Result<PostAuthenticationResponse, AuthenticationError>,
) -> Response {
    if let Err(err) = result {
        if err.redirect_uri.is_none() {
            return response_bad_request();
        }
        match err.flow.unwrap() {
            OidcFlow::Code => {
                let redirect_uri = err.redirect_uri.unwrap();
                let error_body = err.response;
                let mut redirect_uri = Url::parse(&redirect_uri).unwrap();
                redirect_uri
                    .query_pairs_mut()
                    .append_pair("error", error_body.error.to_string().as_str());
                if let Some(state) = error_body.state {
                    redirect_uri.query_pairs_mut().append_pair("state", &state);
                }
                return response_redirect(redirect_uri.as_str());
            }
            OidcFlow::Implicit => {
                let redirect_uri = err.redirect_uri.unwrap();
                let error_body = err.response;
                let mut hash = String::new();
                hash.push_str(&format!(
                    "error={}",
                    http::enc::url_encode::encode(error_body.error.to_string().as_str())
                ));
                if let Some(state) = error_body.state {
                    hash.push_str(&format!("&state={}", http::enc::url_encode::encode(&state)))
                }
                return response_redirect(&format!("{redirect_uri}#{hash}"));
            }
        }
    }

    let result = result.unwrap();

    match result.response {
        AuthenticationResponse::Code(res) => {
            let mut redirect_uri = Url::parse(result.redirect_uri.as_str()).unwrap();

            redirect_uri
                .query_pairs_mut()
                .append_pair("code", &res.code);
            if let Some(ref state) = res.state {
                redirect_uri.query_pairs_mut().append_pair("state", state);
            }
            return response_redirect(redirect_uri.as_str());
        }
        AuthenticationResponse::Implicit(res) => {
            let mut hash = String::new();

            hash.push_str(&format!(
                "id_token={}",
                http::enc::url_encode::encode(&res.id_token)
            ));
            if let Some(ref state) = res.state {
                hash.push_str(&format!("&state={}", http::enc::url_encode::encode(state)));
            }

            let redirect_uri = format!("{}#{}", result.redirect_uri, hash);
            return response_redirect(redirect_uri.as_str());
        }
    }
}
