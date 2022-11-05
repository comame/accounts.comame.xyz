use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::dash::relying_party;
use crate::dash::signin::validate_token;
use crate::http::data::dash_rp_request::{
    RelyingPartyAddRedirectUriRequest, RelyingPartyClientIdRequest,
};
use crate::http::data::dash_rp_response::{RelyingPartiesResponse, RelyingPartyRawSecretResponse};
use crate::http::data::dash_standard_request::StandardRequest;
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

pub async fn list_rp(req: Request<Body>) -> Response<Body> {
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

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::list();

    let response = RelyingPartiesResponse { values: result };

    Response::new(Body::from(to_string(&response).unwrap()))
}

pub async fn create_rp(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<RelyingPartyClientIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::create(&body.client_id);
    if result.is_err() {
        return response_bad_request();
    }
    let result = result.unwrap();

    let response = RelyingPartyRawSecretResponse {
        client_id: result.rp.client_id,
        client_secret: result.raw_secret,
    };

    Response::new(Body::from(to_string(&response).unwrap()))
}

pub async fn update_secret(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<RelyingPartyClientIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::update_secret(&body.client_id);
    if result.is_err() {
        return response_bad_request();
    }

    let result = result.unwrap();

    let response = RelyingPartyRawSecretResponse {
        client_id: result.rp.client_id,
        client_secret: result.raw_secret,
    };

    Response::new(Body::from(to_string(&response).unwrap()))
}

pub async fn delete_rp(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<RelyingPartyClientIdRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::delete(&body.client_id);

    Response::new(Body::from("{}"))
}

pub async fn add_redirect_uri(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<RelyingPartyAddRedirectUriRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let _result = relying_party::add_redirect_uri(&body.client_id, &body.redirect_uri);

    Response::new(Body::from("{}"))
}

pub async fn delete_redirect_uri(req: Request<Body>) -> Response<Body> {
    let body = parse_body(req.into_body()).await;
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();
    let body = from_str::<RelyingPartyAddRedirectUriRequest>(&body);
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::remove_redirect_uri(&body.client_id, &body.redirect_uri);

    Response::new(Body::from("{}"))
}
