use hyper::{Request, Response, Body, StatusCode};
use serde_json::to_string;
use crate::{dash::relying_party, http::data::dash_rp_response::RelyingPartiesResponse};

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn list_rp(req: Request<Body>) -> Response<Body> {
    let result = relying_party::list();

    let response = RelyingPartiesResponse {
        values: result,
    };

    Response::new(Body::from(to_string(&response).unwrap()))
}
