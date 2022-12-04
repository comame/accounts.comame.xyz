use http::{request::Request, response::Response};
use serde_json::to_string;
use url::Url;

use crate::auth::{csrf_token, session};
use crate::data::authentication_failure::AuthenticationFailure;
use crate::data::oidc_flow::authentication_flow_state::OidcFlow;
use crate::data::oidc_flow::authentication_response::AuthenticationResponse;
use crate::data::user_binding::UserBinding;
use crate::oidc::authentication_request::{post_authentication, pronpt_none_fail_authentication};
use crate::web::data::sign_in_continue_request::{
    SignInContinueNoSessionRequest, SignInContinueRequest,
};
use crate::web::data::sign_in_continue_response::SigninContinueSuccessResponse;
use crate::web::get_remote_addr::get_remote_addr;

#[inline]
fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"error": "bad_request"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn response_no_permission() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"error": "no_permission"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn redirect_in_browser(url: &str) -> Response {
    let mut res = Response::new();
    res.body = Some(
        to_string(&SigninContinueSuccessResponse {
            location: url.to_string(),
        })
        .unwrap(),
    );
    res
}

fn redirect(url: &str) -> Response {
    let mut res = Response::new();
    res.status = 302;
    res.headers.insert("Location".to_string(), url.to_string());
    res
}

pub fn handler(req: Request, remote_addr: &str) -> Response {
    let cookie = req.cookies;
    let session_token = cookie.get("Session");
    if session_token.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let session_token = session_token.unwrap().clone();

    let request_body = req.body;
    if request_body.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let request_body = SignInContinueRequest::parse_from(&request_body.unwrap());
    if request_body.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }
    let request_body = request_body.unwrap();

    let user = session::authenticate(
        "id.comame.dev",
        &session_token,
        true,
        &request_body.user_agent_id,
    );
    if user.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let token_ok = csrf_token::validate_once(&request_body.csrf_token);
    if !token_ok {
        dbg!("invalid");
        return response_bad_request();
    }

    let user = user.unwrap();
    let binding_exists = UserBinding::exists(&request_body.relying_party_id, &user.id);
    if binding_exists.is_err() {
        AuthenticationFailure::new(
            &user.id,
            &crate::data::authentication::AuthenticationMethod::Session,
            &crate::data::authentication_failure::AuthenticationFailureReason::NoUserBinding,
            remote_addr,
        );
        dbg!("invalid");
        return response_no_permission();
    }

    let result = post_authentication(
        &user.id,
        &request_body.state_id,
        &request_body.relying_party_id,
        &request_body.user_agent_id,
        request_body.login_type,
        remote_addr,
    );

    if let Err(err) = result {
        if err.redirect_uri.is_none() {
            dbg!("invalid");
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
                return redirect_in_browser(redirect_uri.as_str());
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
                return redirect_in_browser(&format!("{redirect_uri}#{hash}"));
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
            redirect_in_browser(redirect_uri.as_str())
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
            redirect_in_browser(redirect_uri.as_str())
        }
    }
}

pub fn no_interaction_fail(req: Request) -> Response {
    let request_body = req.body;
    if request_body.is_none() {
        return response_bad_request();
    }

    let request_body = SignInContinueNoSessionRequest::parse_from(&request_body.unwrap());
    if request_body.is_err() {
        return response_bad_request();
    }
    let request_body = request_body.unwrap();

    let token_ok = csrf_token::validate_once(&request_body.csrf_token);
    if !token_ok {
        return response_bad_request();
    }

    let result = pronpt_none_fail_authentication(&request_body.state_id);

    if result.redirect_uri.is_none() {
        return response_bad_request();
    }

    match result.flow.unwrap() {
        OidcFlow::Code => {
            let mut redirect_uri = Url::parse(result.redirect_uri.unwrap().as_str()).unwrap();
            redirect_uri
                .query_pairs_mut()
                .append_pair("error", result.response.error.to_string().as_str());
            if let Some(state) = result.response.state {
                redirect_uri
                    .query_pairs_mut()
                    .append_pair("state", state.as_str());
            }

            redirect(redirect_uri.as_str())
        }
        OidcFlow::Implicit => {
            let redirect_uri = result.redirect_uri.unwrap();
            let mut hash = String::new();
            hash.push_str(&format!("error={}", result.response.error));
            if let Some(state) = result.response.state {
                hash.push_str(&format!("&state={state}"));
            }
            redirect(&format!("{redirect_uri}#{hash}"))
        }
    }
}
