use hyper::{Body, Request, Response, StatusCode};

use crate::{auth::session::revoke_session_by_token, http::parse_cookie::parse_cookie};

pub async fn signout(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::from("{}"));

    *response.status_mut() = StatusCode::OK;

    let cookie = req.headers().get("Cookie");
    if cookie.is_none() {
        return response;
    }

    let cookie = parse_cookie(cookie.unwrap().to_str().unwrap());
    if cookie.is_err() {
        return response;
    }

    let cookie_map = cookie.unwrap();
    let session_token = cookie_map.get("Session");
    if session_token.is_none() {
        return response;
    }

    let session_token = session_token.unwrap().clone();
    revoke_session_by_token(&session_token);

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Body, Request, Response, StatusCode};
    use serde_json::{from_str, to_string};

    use crate::{
        auth::{csrf_token::generate, password::set_password},
        data::user::User,
        db::{
            _test_init::{init_mysql, init_redis},
            user::insert_user,
        },
        http::{
            data::password_sign_in_request::PasswordSignInRequest,
            handler::signin::{sign_in_with_password, sign_in_with_session},
            parse_cookie::parse_cookie,
            set_header::set_header_req,
        },
    };

    fn setup_user(user_id: &str) {
        insert_user(&User {
            id: user_id.to_string(),
        })
        .unwrap();
        set_password(user_id, "password");
    }

    #[tokio::test]
    async fn can_signout() {
        init_mysql();
        init_redis();

        let user_id = "http-handler-signout-can_signout";
        setup_user(user_id);

        let csrf_token = generate();
        let req = PasswordSignInRequest {
            user_id: user_id.to_string(),
            password: "password".to_string(),
            csrf_token,
        };
        let req = Request::new(Body::from(to_string(&req).unwrap()));
        let res = sign_in_with_password(req).await;

        let set_cookie_value = &res.headers().get("Set-Cookie").unwrap().to_str().unwrap();
        let set_cookie_value =
            &set_cookie_value[..(set_cookie_value.len() - "; Secure; HttpOnly".len())];
        let cookie = parse_cookie(set_cookie_value).unwrap();
        let session = cookie.get("Session").unwrap().clone();

        let mut req = Request::new(Body::empty());
        set_header_req(&mut req, "Cookie", &format!("Session={}", session));

        let res = signout(req).await;
        assert!(res.status() == StatusCode::OK);

        let mut req = Request::new(Body::empty());
        set_header_req(&mut req, "Cookie", &format!("Session={}", session));
        let res = sign_in_with_session(req).await;
        assert!(res.status() == StatusCode::BAD_REQUEST);
    }
}
