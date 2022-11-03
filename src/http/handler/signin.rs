use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::auth::session::{self, create_session};
use crate::auth::{csrf_token, password};
use crate::data::authentication::{Authentication, LoginPrompt};
use crate::http::data::password_sign_in_request::PasswordSignInRequest;
use crate::http::data::password_sign_in_response::PasswordSignInResponse;
use crate::http::data::session_sign_in_request::SessionSignInRequest;
use crate::http::data::session_sign_in_response::SessionSignInResponse;
use crate::http::parse_body::parse_body;
use crate::http::parse_cookie::parse_cookie;
use crate::http::set_header::set_header;
use crate::http::static_file;

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"error": "bad_request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

#[inline]
fn response_invalid_credential() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"error": "invalid_credential"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

#[inline]
fn response_no_session() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"error": "no_session"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub fn page(name: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let html_file_vec = static_file::read(&format!("/{name}.html")).unwrap();
    let html_file = String::from_utf8(html_file_vec).unwrap();

    let token = csrf_token::generate();

    let replaced_html_file = html_file.replace("$CSRF", token.as_str());

    *response.body_mut() = Body::from(replaced_html_file);

    response
}

pub async fn sign_in_with_password(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
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

    let is_authenticated =
        password::authenticate(&user_id, &password, &audience, LoginPrompt::Login, &ua_id);
    let is_token_collect = csrf_token::validate_keep_token(&token);

    if !is_authenticated {
        return response_invalid_credential();
    }

    if !is_token_collect {
        return response_bad_request();
    }

    let body = PasswordSignInResponse::new(user_id.as_str());
    let mut response = Response::new(Body::from(to_string(&body).unwrap()));

    let session = create_session(&user_id).unwrap();

    let header_value = format!("Session={}; Secure; HttpOnly; Path=/", session.token);
    set_header(&mut response, "Set-Cookie", &header_value);

    response
}

pub async fn sign_in_with_session(req: Request<Body>) -> Response<Body> {
    let cookie = req.headers().get("Cookie");

    if cookie.is_none() {
        return response_no_session();
    }

    let cookie = parse_cookie(cookie.unwrap().to_str().unwrap());

    if cookie.is_err() {
        return response_no_session();
    }

    let cookie_map = cookie.unwrap();
    let session_token = cookie_map.get("Session");

    if session_token.is_none() {
        return response_no_session();
    }

    let body = parse_body(req.into_body()).await;
    if body.is_err() {
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

    Response::new(Body::from(to_string(&body).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::csrf_token::generate;
    use crate::auth::password::set_password;
    use crate::data::user::User;
    use crate::db::_test_init::{init_mysql, init_redis};
    use crate::db::user::insert_user;
    use crate::http::set_header::set_header_req;

    fn setup_user(user_id: &str) {
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        set_password(user_id, "password");
    }

    #[tokio::test]
    async fn correct_password() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signin-correct_password";

        setup_user(user_id);

        let csrf_token = generate();
        let req = PasswordSignInRequest {
            user_id: user_id.to_string(),
            password: "password".to_string(),
            csrf_token,
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::OK);
    }

    #[tokio::test]
    async fn invalid_credential() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signin-invalid_credential";

        setup_user(user_id);

        let csrf_token = generate();

        let req = PasswordSignInRequest {
            user_id: "bob".to_string(),
            password: "password".to_string(),
            csrf_token,
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn invalid_csrf_token() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signin-invalid_csrf_token";

        setup_user(user_id);

        let _csrf_token = generate();

        let req = PasswordSignInRequest {
            user_id: user_id.to_string(),
            password: "password".to_string(),
            csrf_token: "fake".to_string(),
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn password_and_session() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signin-password_and_session";
        setup_user(user_id);

        let csrf_token = generate();
        let req = PasswordSignInRequest {
            user_id: user_id.to_string(),
            password: "password".to_string(),
            csrf_token,
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));
        let res = sign_in_with_password(req).await;

        let set_cookie_value = &res.headers().get("Set-Cookie").unwrap().to_str().unwrap();
        let set_cookie_value =
            &set_cookie_value[..(set_cookie_value.len() - "; Secure; HttpOnly; Path=/".len())];
        let cookie = parse_cookie(set_cookie_value).unwrap();
        let session = cookie.get("Session").unwrap().clone();

        let csrf_token = generate();
        let req = SessionSignInRequest {
            csrf_token,
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };

        let mut req = Request::new(Body::from(to_string(&req).unwrap()));
        set_header_req(&mut req, "Cookie", &format!("Session={}", session));

        let res = sign_in_with_session(req).await;
        assert!(res.status() == StatusCode::OK);
    }

    #[tokio::test]
    async fn invalid_session() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signin-invalid_session";
        setup_user(user_id);

        let csrf_token = generate();
        let req = SessionSignInRequest {
            csrf_token,
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };

        let mut req = Request::new(Body::from(to_string(&req).unwrap()));
        set_header_req(&mut req, "Cookie", &format!("Session={}", "dummy_session"));

        let res = sign_in_with_session(req).await;
        assert!(res.status() == StatusCode::BAD_REQUEST);
    }
}
