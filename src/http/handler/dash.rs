use hyper::{Body, Request, Response, StatusCode};
use url::Url;

use crate::dash::signin;
use crate::http::parse_form_urlencoded::parse;
use crate::http::{set_header, static_file};

#[inline]
fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

pub async fn index(_req: Request<Body>) -> Response<Body> {
    let result = static_file::read("/dash.html").unwrap();
    Response::new(Body::from(String::from_utf8(result).unwrap()))
}

pub async fn signin(_req: Request<Body>) -> Response<Body> {
    let redirect_url = signin::signin();
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::FOUND;
    set_header::set_header(&mut response, "location", &redirect_url);
    response
}

pub async fn callback(req: Request<Body>) -> Response<Body> {
    let uri = Url::parse(&format!("http://examle.com{}", req.uri())).unwrap();
    let query = uri.query();
    if query.is_none() {
        dbg!("invalid");
        return response_bad_request();
    }
    let query = parse(query.unwrap()).unwrap();

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

    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::FOUND;
    set_header::set_header(
        &mut response,
        "location",
        &format!("/dash#{}", token.unwrap()),
    );

    response
}
