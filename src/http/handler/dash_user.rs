use std::env;

use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::dash::user;
use crate::external::session::inspect_token;

use crate::http::data::dash_standard_request::StandardRequest;
use crate::http::data::dash_user_request::{UserIdPasswordRequest, UserIdRequest};
use crate::http::data::dash_user_response::ListUserRespnse;
use crate::http::parse_body::parse_body;

#[inline]
fn response_unauthorized() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "unauthorized"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn list_user(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<StandardRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_unauthorized();
    }

    let result = user::list();

    let response = ListUserRespnse { values: result };

    Response::new(Body::from(to_string(&response).unwrap()))
}

pub async fn create_user(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<UserIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_unauthorized();
    }

    let result = user::create(&body.user_id);
    match result {
        _ => (),
    };

    Response::new(Body::from("{}"))
}

pub async fn delete_user(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<UserIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_unauthorized();
    }

    let result = user::delete(&body.user_id);
    match result {
        _ => (),
    };

    Response::new(Body::from("{}"))
}

pub async fn insert_password(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<UserIdPasswordRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_unauthorized();
    }

    let result = user::insert_password(&body.user_id, &body.password);
    match result {
        _ => (),
    };

    Response::new(Body::from("{}"))
}

pub async fn remove_password(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<UserIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_unauthorized();
    }

    let result = user::remove_password(&body.user_id);
    match result {
        _ => (),
    };

    Response::new(Body::from("{}"))
}