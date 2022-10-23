use hyper::{Body, Request, Response, StatusCode};
use serde_json::to_string;
use url::Url;

use crate::auth::{csrf_token, session};
use crate::data::oidc_flow::authentication_flow_state::OidcFlow;
use crate::data::oidc_flow::authentication_response::AuthenticationResponse;
use crate::enc::url::encode;
use crate::http::data::sign_in_continue_request::{
    SignInContinueNoSessionRequest, SignInContinueRequest,
};
use crate::http::data::sign_in_continue_response::SigninContinueSuccessResponse;
use crate::http::parse_body::parse_body;
use crate::http::parse_cookie::parse_cookie;
use crate::http::set_header::set_header;
use crate::oidc::authentication_request::{post_authentication, pronpt_none_fail_authentication};

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

#[inline]
fn redirect_in_browser(url: &str) -> Response<Body> {
    Response::new(Body::from(
        to_string(&SigninContinueSuccessResponse {
            location: url.to_string(),
        })
        .unwrap(),
    ))
}

fn redirect(url: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::FOUND;
    set_header(&mut response, "location", url);
    response
}

pub async fn handler(req: Request<Body>) -> Response<Body> {
    let cookie = req.headers().get("Cookie");
    if cookie.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let cookie = parse_cookie(cookie.unwrap().to_str().unwrap());
    if cookie.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }

    let cookie = cookie.unwrap();
    let session_token = cookie.get("Session");
    if session_token.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let session_token = session_token.unwrap().clone();

    let user = session::authenticate("id.comame.dev", &session_token, true);
    if user.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let request_body = parse_body(req.into_body()).await;
    if request_body.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }

    let request_body = SignInContinueRequest::parse_from(&request_body.unwrap());
    if request_body.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }
    let request_body = request_body.unwrap();

    let token_ok = csrf_token::validate_once(&request_body.csrf_token);
    if !token_ok {
        dbg!("invalid");
        return response_bad_request();
    }

    let result = post_authentication(
        &user.unwrap().id,
        &request_body.state_id,
        request_body.login_type,
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
                    encode(error_body.error.to_string().as_str())
                ));
                if let Some(state) = error_body.state {
                    hash.push_str(&format!("&state={}", encode(&state)))
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

            hash.push_str(&format!("id_token={}", encode(&res.id_token)));
            if let Some(ref state) = res.state {
                hash.push_str(&format!("&state={}", encode(state)));
            }

            let redirect_uri = format!("{}#{}", result.redirect_uri, hash);
            redirect_in_browser(redirect_uri.as_str())
        }
    }
}

pub async fn no_interaction_fail(req: Request<Body>) -> Response<Body> {
    let request_body = parse_body(req.into_body()).await;
    if request_body.is_err() {
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
            hash.push_str(&format!("error={}", result.response.error.to_string()));
            if let Some(state) = result.response.state {
                hash.push_str(&format!("&state={state}"));
            }
            redirect(&format!("{redirect_uri}#{hash}"))
        }
    }
}
