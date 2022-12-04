use http::request::Request;
use http::response::Response;
use serde_json::{from_str, to_string};

use crate::dash::signin::validate_token;
use crate::dash::user::{self, get_idtoken_issues};
use crate::web::data::dash_standard_request::StandardRequest;
use crate::web::data::dash_user_request::{UserIdPasswordRequest, UserIdRequest};
use crate::web::data::dash_user_response::{IdTokenIssueResponse, ListUserRespnse};

#[inline]
fn response_unauthorized() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"message": "unauthorized"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"message": "Bad Request"}"#.to_string());
    res.status = 403;
    res
}

pub fn list_user(req: Request) -> Response {
    let body = from_str::<StandardRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = user::list();

    let response = ListUserRespnse { values: result };

    let mut res = Response::new();
    res.body = Some(to_string(&response).unwrap());
    res
}

pub fn create_user(req: Request) -> Response {
    let body = from_str::<UserIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let _result = user::create(&body.user_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn delete_user(req: Request) -> Response {
    let body = from_str::<UserIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let _result = user::delete(&body.user_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn insert_password(req: Request) -> Response {
    let body = from_str::<UserIdPasswordRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let _result = user::insert_password(&body.user_id, &body.password);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn remove_password(req: Request) -> Response {
    let body = from_str::<UserIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    user::remove_password(&body.user_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn list_token_issues(req: Request) -> Response {
    let body = from_str::<UserIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let values = get_idtoken_issues(&body.user_id);

    let mut res = Response::new();
    res.body = Some(to_string(&IdTokenIssueResponse { values }).unwrap());
    res
}
