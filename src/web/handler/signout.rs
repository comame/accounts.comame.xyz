use http::{request::Request, response::Response};

use crate::auth::session::revoke_session_by_token;

pub fn signout(req: Request) -> Response {
    let mut response = Response::new();
    response.body = Some("{}".to_string());

    let cookie_map = req.cookies;
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
