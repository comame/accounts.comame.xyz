use hyper::{Body, Request, Response, StatusCode};
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::to_string;
use url::Url;

use crate::data::oidc_flow::id_token_claim::IdTokenClaim;
use crate::http::parse_form_urlencoded;

fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn handler(req: Request<Body>) -> Response<Body> {
    let url = Url::parse(&format!("http://example.com{}", req.uri())).unwrap();

    let query = url.query();
    if query.is_none() {
        return response_bad_request();
    }

    let query = parse_form_urlencoded::parse(query.unwrap());
    if query.is_err() {
        return response_bad_request();
    }
    let query = query.unwrap();

    let id_token = query.get("id_token").cloned();
    if id_token.is_none() {
        return response_bad_request();
    }

    let jwt = jsonwebtoken::decode::<IdTokenClaim>(
        &id_token.unwrap(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    );
    if jwt.is_err() {
        return response_bad_request();
    }
    let jwt = jwt.unwrap().claims;

    let jwt_string = to_string(&jwt);

    Response::new(Body::from(jwt_string.unwrap()))
}
