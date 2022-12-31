use http::{request::Request, response::Response};

use crate::oidc::relying_party::generate_authentication_endpoint_url;

pub fn handler(_req: &Request) -> Response {
    let redirect_url = generate_authentication_endpoint_url();

    let mut res = Response::new();
    res.status = 302;
    res.headers.insert("Location".into(), redirect_url);

    res
}
