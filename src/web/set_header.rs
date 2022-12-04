use std::borrow::BorrowMut;

use http::response::Response;
use hyper::http::HeaderValue;
use hyper::{Request as HyperRequest, Response as HyperResponse};

// Used in tests
#[deprecated]
pub fn set_header_req<T>(req: &mut HyperRequest<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    req.headers_mut().append(key, header_value);
}

pub fn no_store(res: &mut Response) {
    res.headers
        .borrow_mut()
        .insert("Cache-Control".to_string(), "no-store".to_string());
    res.headers
        .borrow_mut()
        .insert("Pragma".to_string(), "no-cache".to_string());
}
