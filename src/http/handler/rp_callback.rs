use hyper::{Body, Request, Response, StatusCode};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::to_string;
use url::Url;

use crate::data::oidc_flow::id_token_claim::IdTokenClaim;
use crate::http::{parse_form_urlencoded, static_file};

pub async fn handler(_req: Request<Body>) -> Response<Body> {
    let result = static_file::read("/callback.html");
    Response::new(Body::from(String::from_utf8(result.unwrap()).unwrap()))
}
