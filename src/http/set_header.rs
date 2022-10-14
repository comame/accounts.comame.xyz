use hyper::http::HeaderValue;
use hyper::{Request, Response};

pub fn set_header<T>(res: &mut Response<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    res.headers_mut().append(key, header_value);
}

#[allow(dead_code)]
// Used in tests
pub fn set_header_req<T>(req: &mut Request<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    req.headers_mut().append(key, header_value);
}
