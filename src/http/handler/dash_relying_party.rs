use std::env;

use hyper::{Body, Request, Response, StatusCode};
use serde_json::{from_str, to_string};

use crate::dash::relying_party;
use crate::external::session::inspect_token;
use crate::http::data::dash_rp_request::RelyingPartyClientIdRequest;
use crate::http::data::dash_rp_response::{RelyingPartiesResponse, RelyingPartyRawSecretResponse};
use crate::http::data::dash_standard_request::StandardRequest;
use crate::http::parse_body::parse_body;

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

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_bad_request();
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

    let user = inspect_token(
        "accounts.comame.xyz",
        &env::var("CLIENT_SECRET").unwrap(),
        &body.token,
    );
    if user.is_none() {
        return response_bad_request();
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
