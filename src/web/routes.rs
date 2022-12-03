use http::{
    hyper::{from_hyper_request, from_hyper_request_without_body, to_hyper_response},
    request::Method,
};
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};

use crate::web::handler;

pub async fn routes(hyper_request: HyperRequest<Body>) -> HyperResponse<Body> {
    let req = from_hyper_request_without_body(&hyper_request);

    let response = match (req.method, req.path.as_ref()) {
        (Method::Get, "/authenticate") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_authentication_request::handler(req)
        }
        (Method::Post, "/authenticate") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_authentication_request::handler(req)
        }
        _ => return crate::web::old_routes::routes(hyper_request).await,
    };

    to_hyper_response(response)
}
