use http::request::Request;
use http::response::Response;
use serde_json::to_string;

use crate::data::oidc_flow::code_request::CodeRequest;
use crate::enc::basic_auth::BasicAuthorization;
use crate::oidc::code_request::code_request;
use crate::web::set_header::no_store;

fn response_bad_request() -> Response {
    let mut response = Response::new();
    response.body = Some(r#"{"error": "invalid_request"}"#.to_string());
    response.status = 403;
    response
        .headers
        .insert("Content-Type".into(), "application/json".into());
    response
}

pub fn handle(req: &Request) -> Response {
    let req = req.clone();
    let basic_auth_result = req.headers.get("Authorization").cloned();
    let mut user: Option<String> = None;
    let mut password: Option<String> = None;

    if let Some(result) = basic_auth_result {
        let value = BasicAuthorization::decode(&result);
        if let Ok(auth) = value {
            user = Some(auth.user);
            password = Some(auth.password)
        }
    }

    if req.body.is_none() {
        return response_bad_request();
    }

    let body = CodeRequest::parse(&req.body.unwrap(), user.as_deref(), password.as_deref());
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

    let mut response = Response::new();
    response.body = Some(to_string(&result).unwrap());
    response
        .headers
        .insert("Content-Type".into(), "application/json".into());
    no_store(&mut response);
    response
}
