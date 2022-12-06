use http::request::Request;
use http::response::Response;

use crate::dash::signin;
use crate::web::static_file;

#[inline]
fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"message": "Bad Request"}"#.to_string());
    res.status = 403;
    res
}

pub fn index(_req: &Request) -> Response {
    let result = static_file::read("/dash.html").unwrap();
    let mut res = Response::new();
    res.body = Some(String::from_utf8(result).unwrap());
    res
}

pub fn signin(_req: &Request) -> Response {
    let redirect_url = signin::signin();
    let mut res = Response::new();
    res.status = 302;
    res.headers.insert("Location".to_string(), redirect_url);
    res
}

pub async fn callback(req: &Request) -> Response {
    let query = req.query.clone();
    if query.is_none() {
        return response_bad_request();
    }
    let query = http::enc::form_urlencoded::parse(&query.unwrap()).unwrap();

    let state = query.get("state");
    if state.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let code = query.get("code");
    if code.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }

    let token = signin::callback(state.unwrap(), code.unwrap()).await;
    if token.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }

    let mut response = Response::new();
    response.status = 302;
    response
        .headers
        .insert("Location".to_string(), format!("/dash#{}", token.unwrap()));

    response
}
