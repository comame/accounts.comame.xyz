use http::{request::Request, response::Response};
use serde_json::{from_str, to_string};

use crate::auth::session::{self, create_session};
use crate::auth::{csrf_token, password};
use crate::data::authentication::{Authentication, LoginPrompt};
use crate::web::data::password_sign_in_request::PasswordSignInRequest;
use crate::web::data::password_sign_in_response::PasswordSignInResponse;
use crate::web::data::session_sign_in_request::SessionSignInRequest;
use crate::web::data::session_sign_in_response::SessionSignInResponse;
use crate::web::get_remote_addr::get_remote_addr;
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

    let html_file_vec = static_file::read(&format!("/{name}.html")).unwrap();
    let html_file = String::from_utf8(html_file_vec).unwrap();

    let token = csrf_token::generate();

    let replaced_html_file = html_file.replace("$CSRF", token.as_str());

    response.body = Some(replaced_html_file);

    response
}

pub fn sign_in_with_password(req: Request, remote_addr: &str) -> Response {
    let body = req.body;
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
        &remote_addr,
    );
    let is_token_collect = csrf_token::validate_keep_token(&token);

    if !is_authenticated {
        return response_invalid_credential();
    }

    if !is_token_collect {
        return response_bad_request();
    }

    let session = create_session(&user_id).unwrap();
    let body = PasswordSignInResponse::new(user_id.as_str());

    let mut res = Response::new();
    res.body = Some(to_string(&body).unwrap());
    res.cookies
        .push(http::cookies::build("Session", &session.token).build());

    res
}

pub fn sign_in_with_session(req: Request) -> Response {
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

    let user = session::authenticate(
        &request.relying_party_id,
        session_token.unwrap(),
        false,
        &request.user_agent_id,
    );

    if user.is_none() {
        return response_no_session();
    }

    let user = user.unwrap();

    let latest_authentication = Authentication::latest(&user.id, &request.user_agent_id);

    let csrf_token_correct = csrf_token::validate_keep_token(&request.csrf_token);

    if !csrf_token_correct {
        return response_bad_request();
    }

    let body = SessionSignInResponse {
        user_id: user.id,
        last_auth: latest_authentication.map(|auth| auth.authenticated_at),
    };

    let mut res = Response::new();
    res.body = Some(to_string(&body).unwrap());
    res
}
