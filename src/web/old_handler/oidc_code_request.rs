use hyper::{Body, Request, Response, StatusCode};
use serde_json::to_string;

use crate::data::oidc_flow::code_request::CodeRequest;
use crate::enc::basic_auth::BasicAuthorization;
use crate::oidc::code_request::code_request;
use crate::web::parse_body::parse_body;

fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"error": "invalid_request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn handle(req: Request<Body>) -> Response<Body> {
    let basic_auth_result = req.headers().get("Authorization").cloned();
    let mut user: Option<String> = None;
    let mut password: Option<String> = None;

    if let Some(result) = basic_auth_result {
        let value = result.to_str().unwrap();
        let value = BasicAuthorization::decode(value);
        if let Ok(auth) = value {
            user = Some(auth.user);
            password = Some(auth.password)
        }
    }

    let body = parse_body(req.into_body()).await.unwrap();

    let body = CodeRequest::parse(&body, user.as_deref(), password.as_deref());
    if body.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }
    let body = body.unwrap();

    let result = code_request(body);
    if result.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }
    let result = result.unwrap();

    Response::new(Body::from(to_string(&result).unwrap()))
}
