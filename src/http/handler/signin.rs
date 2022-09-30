use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::auth::csrf_token;
use crate::auth::password::authenticated;

use crate::http::data::sign_in_request::SignInRequest;
use crate::http::data::sign_in_response::SignInResponse;
use crate::http::parse_body::parse_body;
use crate::http::static_file;

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from("{}"));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub fn page() -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let html_file_vec = static_file::read("/sign-in.html").unwrap();
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

    let request = match from_str::<SignInRequest>(body.unwrap().as_str()) {
        Ok(v) => v,
        Err(_) => {
            return response_bad_request();
        }
    };

    let user_id = request.user_id;
    let password = request.password;
    let token = request.csrf_token;

    let is_authenticated = authenticated(&user_id, &password);
    let is_token_collect = csrf_token::validate(&token);

    if !(is_authenticated && is_token_collect) {
        return response_bad_request();
    }

    let res = SignInResponse::new(user_id.as_str());

    Response::new(Body::from(to_string(&res).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth::{csrf_token::generate, password::set_password},
        data::user::User,
        db::{
            _test_init::{init_mysql, init_redis},
            user::insert_user,
        },
    };

    fn setup_user() {
        insert_user(&User {
            id: "user".to_string(),
        })
        .unwrap();
        set_password("user", "password");
    }

    #[tokio::test]
    #[ignore = "Single thread only"]
    async fn single_thread_correct() {
        init_mysql();
        init_redis();
        setup_user();

        let csrf_token = generate();
        let req = SignInRequest {
            user_id: "user".to_string(),
            password: "password".to_string(),
            csrf_token,
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::OK);
    }

    #[tokio::test]
    #[ignore = "Single thread only"]
    async fn single_thread_invalid_credential() {
        init_mysql();
        init_redis();
        setup_user();

        let csrf_token = generate();

        let req = SignInRequest {
            user_id: "bob".to_string(),
            password: "password".to_string(),
            csrf_token,
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[ignore = "Single thread only"]
    async fn single_thread_invalid_csrf_token() {
        init_mysql();
        init_redis();
        setup_user();

        let _csrf_token = generate();

        let req = SignInRequest {
            user_id: "user".to_string(),
            password: "password".to_string(),
            csrf_token: "fake".to_string(),
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));

        let res = sign_in_with_password(req).await;

        assert!(res.status() == StatusCode::BAD_REQUEST);
    }
}
