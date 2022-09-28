use hyper::{Response, Body, http::HeaderValue, StatusCode};

pub fn moved_permanently(path: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let header_value = HeaderValue::from_str(path).unwrap();

    *response.status_mut() = StatusCode::MOVED_PERMANENTLY;
    response.headers_mut().append("Location", header_value);

    response
}
