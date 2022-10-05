use hyper::{http::HeaderValue, Body, Response, StatusCode};

use super::set_header::set_header;

pub fn moved_permanently(path: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    *response.status_mut() = StatusCode::MOVED_PERMANENTLY;

    set_header(&mut response, "Location", path);

    response
}
