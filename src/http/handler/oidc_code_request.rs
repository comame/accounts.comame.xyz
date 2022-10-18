use hyper::{Body, Request, Response, StatusCode};
use serde_json::to_string;

use crate::data::oidc_flow::code_request::CodeRequest;
use crate::http::parse_body::parse_body;
use crate::oidc::code_request::code_request;

fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"error": "invalid_request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn handle(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await.unwrap();
    let body = CodeRequest::parse(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let result = code_request(body);
    if result.is_err() {
        return response_bad_request();
    }
    let result = result.unwrap();

    Response::new(Body::from(to_string(&result).unwrap()))
}
