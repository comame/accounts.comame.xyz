use hyper::{Body, Request, Response, StatusCode};
use url::Url;

use crate::auth::session::revoke_session_by_token;
use crate::web::set_header;

pub async fn signout(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::from("{}"));

    *response.status_mut() = StatusCode::OK;

    let cookie = req.headers().get("Cookie");
    if cookie.is_none() {
        return response;
    }

    let cookie = http::cookies::parse(cookie.unwrap().to_str().unwrap());
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

    let uri = Url::parse(&format!("http://example.com{}", req.uri())).unwrap();
    let query = uri.query();

    if query.is_none() {
        return response;
    }

    let query = query.unwrap();
    let query_map = http::enc::form_urlencoded::parse(query);

    if query_map.is_err() {
        return response;
    }
    let query_map = query_map.unwrap();

    let continue_uri = query_map.get("continue");
    if continue_uri.is_none() {
        return response;
    }

    *response.status_mut() = StatusCode::FOUND;
    set_header::set_header(&mut response, "Location", continue_uri.unwrap().as_str());

    response
}

#[cfg(test)]
mod tests {
    use hyper::{Body, Request, StatusCode};
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
        let req = Request::new(Body::from(to_string(&req).unwrap()));
        let res = sign_in_with_password(req).await;

        let set_cookie_value = &res.headers().get("Set-Cookie").unwrap().to_str().unwrap();
        let cookie = http::cookies::for_test::parse_set_cookie(&set_cookie_value).unwrap();
        let session = cookie.1;

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
