use http::request::Request;
use http::response::Response;
use serde_json::{from_str, to_string};

use crate::external::verfy_id_token::verify_id_token;
use crate::web::data::tools_id_token::{IdTokenRequest, IdTokenResponse};

fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.status = 403;
    res.body = Some(r#"{"error": "invalid_request"}"#.to_string());
    res
}

pub fn handle(req: &Request) -> Response {
    let body = req.body.clone();
    if body.is_none() {
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

    let response_body = to_string(&IdTokenResponse { claim }).unwrap();

    let mut res = Response::new();
    res.body = Some(response_body);
    res
}
