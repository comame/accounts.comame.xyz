use http::request::Request;
use http::response::Response;
use serde_json::to_string;

use crate::oidc::userinfo::{userinfo, ErrorReason};

fn response_error(error: &str) -> Response {
    let mut response = Response::new();
    response.status = 401;
    response
        .headers
        .insert("Content-Type".into(), "application/json".into());
    response.headers.insert(
        "WWW-Authenticate".to_string(),
        format!(r#"error="{}""#, error),
    );
    response.body = Some("{}".into());
    response
}

pub fn handle(req: &Request) -> Response {
    let mut token = String::new();

    let authorization_header_value = req.headers.get("Authorization").cloned();
    if let Some(header) = authorization_header_value {
        let value = &header;
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
            ErrorReason::InvalidToken => "invalid_token",
        };
        return response_error(message);
    }

    let result = result.unwrap();

    let mut res = Response::new();
    res.headers
        .insert("Content-Type".into(), "application/json".into());
    res.body = Some(to_string(&result).unwrap());

    res
}
