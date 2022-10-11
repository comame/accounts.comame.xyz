use hyper::{Request, Body, Response, StatusCode};
use crate::{http::{parse_cookie::parse_cookie, parse_body::parse_body, data::sign_in_continue_request::SignInContinueRequest}, data::{authentication::LoginPrompt}, auth::{session, csrf_token}};

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(""));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn handler(req: Request<Body>) -> Response<Body> {
    let cookie = req.headers().get("Cookie");
    if cookie.is_none() {
        return response_bad_request();
    }

    let cookie = parse_cookie(cookie.unwrap().to_str().unwrap());
    if cookie.is_err() {
        return response_bad_request();
    }

    let cookie = cookie.unwrap();
    let session_token = cookie.get("Session");
    if session_token.is_none() {
        return response_bad_request();
    }

    let session_token = session_token.unwrap().clone();

    let user = session::authenticate("id.comame.dev", &session_token, LoginPrompt::Login, true);
    if user.is_none() {
        return response_bad_request();
    }

    let request_body = parse_body(req.into_body()).await;
    if request_body.is_err() {
        return response_bad_request();
    }

    let request_body = SignInContinueRequest::parse_from(&request_body.unwrap());
    if request_body.is_err() {
        return response_bad_request();
    }

    let token_ok = csrf_token::validate_once(&request_body.unwrap().csrf_token);
    if !token_ok {
        return response_bad_request();
    }

    Response::new(Body::from(format!("You are signed as {}.", user.unwrap().id)))
}
