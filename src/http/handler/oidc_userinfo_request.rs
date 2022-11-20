use hyper::{http::HeaderValue, Body, Request, Response, StatusCode};
use serde_json::to_string;

use crate::oidc::userinfo::{userinfo, ErrorReason};

fn response_error(error: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let header_value = format!(r#"error="{}""#, error);

    response.headers_mut().append(
        "WWW-Authenticate",
        HeaderValue::from_str(&header_value).unwrap(),
    );
    *response.status_mut() = StatusCode::UNAUTHORIZED;

    response
}

pub async fn handle(req: Request<Body>) -> Response<Body> {
    let mut token = String::new();

    let authorization_header_value = req.headers().get("Authorization").cloned();
    if let Some(header) = authorization_header_value {
        let value = header.to_str().unwrap();
        if value.len() > "Bearer ".len() {
            token = value["Bearer ".len()..].to_string();
        }
    }

    // TODO: クエリにも対応する

    if token.is_empty() {
        return response_error("invalid_request");
    }

    let result = userinfo(&token);

    if let Err(error) = result {
        let message = match error {
            ErrorReason::InsufficientScope => "insufficient_scope",
            ErrorReason::InvalidToken => "invalid_token",
        };
        return response_error(message);
    }

    let result = result.unwrap();

    Response::new(Body::from(to_string(&result).unwrap()))
}
