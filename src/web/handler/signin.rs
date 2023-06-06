use http::request::Request;
use http::response::Response;
use serde_json::{from_str};

use crate::auth::session::{self, create_session};
use crate::auth::{csrf_token, password};
use crate::data::authentication::{AuthenticationMethod, LoginPrompt};
use crate::data::authentication_failure::{AuthenticationFailure, AuthenticationFailureReason};
use crate::data::role_access::RoleAccess;
use crate::oidc::authentication_request::{post_authentication, response_post_authentication};
use crate::web::data::password_sign_in_request::PasswordSignInRequest;
use crate::web::data::session_sign_in_request::SessionSignInRequest;
use crate::web::static_file;

#[inline]
fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"error": "bad_request"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn response_invalid_credential() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"error": "invalid_credential"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn response_no_session() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"error": "no_session"}"#.to_string());
    res.status = 403;
    res
}

pub fn page(name: &str) -> Response {
    let mut response = Response::new();

    let html_file = static_file::read(&format!("/{name}.html")).unwrap();

    let token = csrf_token::generate();

    let replaced_html_file = html_file.replace("$CSRF", token.as_str());

    response.body = Some(replaced_html_file);

    response
}

pub fn sign_in_with_password(req: &Request, remote_addr: &str) -> Response {
    let body = req.body.clone();
    if body.is_none() {
        return response_bad_request();
    }

    let request = match from_str::<PasswordSignInRequest>(body.unwrap().as_str()) {
        Ok(v) => v,
        Err(_) => {
            return response_bad_request();
        }
    };

    let user_id = request.user_id;
    let password = request.password;
    let token = request.csrf_token;
    let audience = request.relying_party_id;
    let ua_id = request.user_agent_id;

    let is_authenticated = password::authenticate(
        &user_id,
        &password,
        &audience,
        LoginPrompt::Login,
        &ua_id,
        remote_addr,
    );
    let is_token_collect = csrf_token::validate_once(&token);

    if !is_authenticated {
        return response_invalid_credential();
    }

    if !is_token_collect {
        return response_bad_request();
    }

    let is_accessible = RoleAccess::is_accessible(&user_id, &audience);
    if !is_accessible {
        AuthenticationFailure::new(
            &user_id,
            &AuthenticationMethod::Session,
            &AuthenticationFailureReason::NoUserBinding,
            remote_addr,
        );
        dbg!("invalid");
        return response_bad_request();
    }

    let result = post_authentication(
        &user_id,
        &request.state_id,
        &audience,
        &ua_id,
        AuthenticationMethod::Password,
        remote_addr,
    );

    let mut res = response_post_authentication(result);

    let session = create_session(&user_id).unwrap();
    res.cookies
        .push(http::cookies::build("Session", &session.token).build());

    res
}

pub fn sign_in_with_session(req: &Request, remote_address: &str) -> Response {
    let req = req.clone();

    let cookie_map = req.cookies;
    let session_token = cookie_map.get("Session");

    if session_token.is_none() {
        return response_no_session();
    }

    let body = req.body;
    if body.is_none() {
        return response_no_session();
    }

    let request = match from_str::<SessionSignInRequest>(&body.unwrap()) {
        Ok(v) => v,
        Err(_) => {
            return response_no_session();
        }
    };

    let user = session::authenticate(&request.relying_party_id, session_token.unwrap());

    if user.is_none() {
        return response_no_session();
    }

    let user = user.unwrap();

    let csrf_token_correct = csrf_token::validate_once(&request.csrf_token);

    if !csrf_token_correct {
        return response_bad_request();
    }

    let is_accessible = RoleAccess::is_accessible(&user.id, &request.relying_party_id);
    if !is_accessible {
        AuthenticationFailure::new(
            &user.id,
            &AuthenticationMethod::Session,
            &AuthenticationFailureReason::NoUserBinding,
            remote_address,
        );
        dbg!("invalid");
        return response_bad_request();
    }

    let result = post_authentication(
        &user.id,
        &request.state_id,
        &request.relying_party_id,
        &request.user_agent_id,
        AuthenticationMethod::Session,
        remote_address,
    );

    response_post_authentication(result)
}
