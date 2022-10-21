use hyper::{Body, Request, Response, StatusCode};
use url::Url;

use crate::auth::{csrf_token, session};
use crate::data::oidc_flow::authentication_response::AuthenticationResponse;
use crate::http::data::sign_in_continue_request::{
    SignInContinueNoSessionRequest, SignInContinueRequest,
};
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
fn redirect(url: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::FOUND;
    set_header(&mut response, "Location", url);
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
        let redirect_uri = err.redirect_uri.unwrap();
        let error_body = err.response;
        let mut redirect_uri = Url::parse(&redirect_uri).unwrap();
        redirect_uri
            .query_pairs_mut()
            .append_pair("error", error_body.error.to_string().as_str());
        if let Some(state) = error_body.state {
            redirect_uri.query_pairs_mut().append_pair("state", &state);
        }
        return redirect(redirect_uri.as_str());
    }

    let result = result.unwrap();
    let mut redirect_uri = Url::parse(result.redirect_uri.as_str()).unwrap();

    match result.response {
        AuthenticationResponse::Code(res) => {
            redirect_uri
                .query_pairs_mut()
                .append_pair("code", &res.code);
            if let Some(ref state) = res.state {
                redirect_uri.query_pairs_mut().append_pair("state", state);
            }
        }
        AuthenticationResponse::Implicit(res) => {
            redirect_uri
                .query_pairs_mut()
                .append_pair("id_token", &res.id_token);
            if let Some(ref state) = res.state {
                redirect_uri.query_pairs_mut().append_pair("state", state);
            }
        }
    }

    redirect(redirect_uri.as_str())
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
