use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::auth::password::authenticated;
use crate::crypto::rand;
use crate::http::data::sign_in_request::SignInRequest;
use crate::http::data::sign_in_response::SignInResponse;
use crate::http::parse_body::parse_body;
use crate::http::static_file;

pub fn page() -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let html_file_vec = static_file::read("/sign-in.html").unwrap();
    let html_file = String::from_utf8(html_file_vec).unwrap();

    let token = rand::random_str(32);

    let replaced_html_file = html_file.replace("$CSRF", token.as_str());

    *response.body_mut() = Body::from(replaced_html_file);

    response
}

pub async fn sign_in(req: Request<Body>) -> Response<Body> {
    let mut has_error = false;

    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        has_error = true;
    }

    let body = from_str::<SignInRequest>(body.unwrap().as_str());
    if body.is_err() {
        has_error = true;
    }
    let body = body.unwrap();

    let user_id = body.user_id;
    let password = body.password;

    let is_authenticated = authenticated(&user_id.as_str(), &password.as_str());

    dbg!(is_authenticated);

    has_error = has_error || !is_authenticated;

    if has_error {
        let mut response = Response::new(Body::from("{}"));
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return response;
    }

    let res = SignInResponse::new(&user_id.as_str());

    let response = Response::new(Body::from(to_string(&res).unwrap()));

    response
}
