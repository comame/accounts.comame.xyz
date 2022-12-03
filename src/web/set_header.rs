use std::borrow::BorrowMut;

use http::response::Response;
use hyper::http::HeaderValue;
use hyper::{Request as HyperRequest, Response as HyperResponse};

#[deprecated]
pub fn set_header<T>(res: &mut HyperResponse<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    res.headers_mut().append(key, header_value);
}

// Used in tests
#[deprecated]
pub fn set_header_req<T>(req: &mut HyperRequest<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    req.headers_mut().append(key, header_value);
}

#[deprecated]
pub fn set_no_store_old<T>(res: &mut HyperResponse<T>) {
    let no_store = HeaderValue::from_str("no-store").unwrap();
    let no_cache = HeaderValue::from_str("no-cache").unwrap();

    res.headers_mut().append("Cache-Control", no_store);
    res.headers_mut().append("Pragma", no_cache);
}

pub fn no_store(res: &mut Response) {
    res.headers
        .borrow_mut()
        .insert("Cache-Control".to_string(), "no-store".to_string());
    res.headers
        .borrow_mut()
        .insert("Pragma".to_string(), "no-cache".to_string());
}
