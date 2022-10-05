use hyper::{http::HeaderValue, Response};

pub fn set_header<T>(res: &mut Response<T>, key: &'static str, value: &str) {
    let header_value = HeaderValue::from_str(value).unwrap();
    res.headers_mut().append(key.clone(), header_value);
}
