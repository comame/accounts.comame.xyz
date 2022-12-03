use std::collections::HashMap;
use std::str::FromStr;

use hyper::body::to_bytes;
use hyper::header::HeaderName;
use hyper::http::HeaderValue;
use hyper::{
    Body as HyperBody, Method as HyperMethod, Request as HyperRequest, Response as HyperResponse,
    StatusCode,
};

use crate::request::{Method, Request};
use crate::response::Response;

async fn parse_body(body: HyperBody) -> Result<String, ()> {
    let bytes = to_bytes(body).await;
    if let Err(err) = bytes {
        eprintln!("{}", err);
        return Err(());
    }

    let vec = bytes.unwrap().iter().cloned().collect::<Vec<u8>>();

    let str = String::from_utf8(vec);
    if let Err(err) = str {
        eprintln!("{}", err);
        return Err(());
    }

    Ok(str.unwrap())
}

pub fn from_hyper_request_without_body(request: &HyperRequest<HyperBody>) -> Request {
    let method = match request.method() {
        &HyperMethod::GET => Method::Get,
        &HyperMethod::POST => Method::Post,
        _ => todo!(),
    };

    let mut headers = HashMap::new();
    for key in request.headers().keys() {
        let value = request
            .headers()
            .get(key)
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_string();
        headers.insert(key.to_string(), value);
    }

    let uri = request.uri().clone();

    let cookie_header_value = headers.get("Cookie");
    let cookies = if let Some(cookie) = cookie_header_value {
        crate::cookies::parse(cookie).unwrap()
    } else {
        HashMap::new()
    };

    Request {
        method,
        path: uri.path().to_string(),
        query: uri.query().map(|s| s.to_string()),
        headers,
        cookies,
        body: None,
    }
}

pub async fn from_hyper_request(hyper_request: HyperRequest<HyperBody>) -> Request {
    let mut request = from_hyper_request_without_body(&hyper_request);
    request.body = Some(parse_body(hyper_request.into_body()).await.unwrap());
    request
}

pub fn to_hyper_response(response: Response) -> HyperResponse<HyperBody> {
    let body = if let Some(body) = response.body {
        HyperBody::from(body)
    } else {
        HyperBody::empty()
    };
    let mut hyper_response = HyperResponse::new(body);

    *hyper_response.status_mut() = StatusCode::from_u16(response.status).unwrap();

    for (key, value) in response.headers {
        let header_key = HeaderName::from_str(&key).unwrap();
        let header_value = HeaderValue::from_str(&value).unwrap();
        hyper_response
            .headers_mut()
            .append(header_key, header_value);
    }

    for cookie in response.cookies {
        let header_value = HeaderValue::from_str(&cookie).unwrap();
        hyper_response
            .headers_mut()
            .append("Set-Cookie", header_value);
    }

    hyper_response
}
