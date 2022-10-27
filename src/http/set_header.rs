use hyper::http::HeaderValue;
use hyper::{Request, Response};

pub fn set_header<T>(res: &mut Response<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    res.headers_mut().append(key, header_value);
}

// Used in tests
pub fn set_header_req<T>(req: &mut Request<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    req.headers_mut().append(key, header_value);
}

pub fn set_no_store<T>(res: &mut Response<T>) {
    let no_store = HeaderValue::from_str("no-store").unwrap();
    let no_cache = HeaderValue::from_str("no-cache").unwrap();

    res.headers_mut().append("Cache-Control", no_store);
    res.headers_mut().append("Pragma", no_cache);
}
