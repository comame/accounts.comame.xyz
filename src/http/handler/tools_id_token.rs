use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::external::session::create_session;
use crate::external::verfy_id_token::verify_id_token;
use crate::http::data::tools_id_token::{IdTokenRequest, IdTokenResponse};
use crate::http::parse_body::parse_body;

fn response_bad_request() -> Response<Body> {
    let mut res = Response::new(Body::from(r#"{"error": "invalid_request"}"#));
    *res.status_mut() = StatusCode::BAD_REQUEST;
    res
}

pub async fn handle(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let body = from_str::<IdTokenRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let claim = verify_id_token(&body.client_id, &body.client_secret, &body.id_token);
    if claim.is_err() {
        return response_bad_request();
    }
    let claim = claim.unwrap();

    let session = create_session(&body.client_id, &body.client_secret, &claim.sub);
    if session.is_err() {
        return response_bad_request();
    }
    let session = session.unwrap();

    let response_body = to_string(&IdTokenResponse { claim, session }).unwrap();

    Response::new(Body::from(response_body))
}
