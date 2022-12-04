use http::{request::Request, response::Response};

use crate::auth::session::revoke_session_by_token;

pub fn signout(req: Request) -> Response {
    let mut response = Response::new();
    response.body = Some("{}".to_string());

    let cookie = req.headers.get("Cookie");
    if cookie.is_none() {
        return response;
    }

    let cookie = http::cookies::parse(cookie.unwrap());
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

    let query = req.query;

    if query.is_none() {
        return response;
    }

    let query = query.unwrap();
    let query_map = http::enc::form_urlencoded::parse(&query);

    if query_map.is_err() {
        return response;
    }
    let query_map = query_map.unwrap();

    let continue_uri = query_map.get("continue");
    if continue_uri.is_none() {
        return response;
    }

    let mut res = Response::new();
    res.status = 302;
    res.headers
        .insert("Location".to_string(), continue_uri.unwrap().to_string());
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::request::Method;
    use http::{request::Request, response::Response};
    use hyper::{Body, Request as HyperRequest, StatusCode};
    use serde_json::to_string;

    use super::*;
    use crate::auth::csrf_token::generate;
    use crate::auth::password::set_password;
    use crate::data::user::User;
    use crate::db::_test_init::{init_mysql, init_redis};
    use crate::db::user::insert_user;
    use crate::web::data::password_sign_in_request::PasswordSignInRequest;
    use crate::web::old_handler::signin::{sign_in_with_password, sign_in_with_session};
    use crate::web::set_header::set_header_req;

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
            relying_party_id: "rp.comame.dev".to_string(),
            user_agent_id: "ua".to_string(),
        };
        let req = HyperRequest::new(Body::from(to_string(&req).unwrap()));
        let res = sign_in_with_password(req).await;

        let set_cookie_value = &res.headers().get("Set-Cookie").unwrap().to_str().unwrap();
        let cookie = http::cookies::for_test::parse_set_cookie(&set_cookie_value).unwrap();
        let session = cookie.1;

        let mut cookies = HashMap::new();
        cookies.insert("Session".to_string(), session.clone());
        let res = signout(Request {
            path: "".to_string(),
            method: Method::Get,
            headers: HashMap::new(),
            query: None,
            cookies,
            body: Some("".to_string()),
        });
        assert!(res.status == 200);

        let mut req = HyperRequest::new(Body::empty());
        set_header_req(&mut req, "Cookie", &format!("Session={}", session));
        let res = sign_in_with_session(req).await;
        assert!(res.status() == StatusCode::BAD_REQUEST);
    }
}
