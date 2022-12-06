use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;

use hyper::body::to_bytes;
use hyper::header::HeaderName;
use hyper::http::HeaderValue;
use hyper::{
    Body as HyperBody, Method as HyperMethod, Request as HyperRequest, Response as HyperResponse,
    StatusCode,
};
use tokio::join;

use crate::enc::normalize_header_key::normalize_header_key;
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

impl Request {
    async fn from_async(request: HyperRequest<HyperBody>) -> Request {
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
            headers.insert(normalize_header_key(key.as_str()), value);
        }

        let uri = request.uri().clone();

        let cookie_header_value = headers.get("Cookie");
        let cookies = if let Some(cookie) = cookie_header_value {
            crate::cookies::parse(cookie).unwrap()
        } else {
            HashMap::new()
        };

        let request = Request {
            method,
            path: uri.path().to_string(),
            query: uri.query().map(|s| s.to_string()),
            headers,
            cookies,
            body: Some(parse_body(request.into_body()).await.unwrap()),
        };

        request
    }
}

pub struct RequestAsync {
    get: Pin<Box<dyn Future<Output = Request> + Send + Sync>>,
}

impl RequestAsync {
    pub async fn get(self) -> Request {
        join!(self.get).0
    }
}

// From trait implementations below

impl From<HyperRequest<HyperBody>> for RequestAsync {
    fn from(request: HyperRequest<HyperBody>) -> Self {
        RequestAsync {
            get: Box::pin(Request::from_async(request)),
        }
    }
}

impl From<HyperResponse<HyperBody>> for Response {
    fn from(_: HyperResponse<HyperBody>) -> Self {
        unimplemented!()
    }
}

impl From<Request> for HyperRequest<HyperBody> {
    fn from(_: Request) -> Self {
        unimplemented!()
    }
}

impl From<Response> for HyperResponse<HyperBody> {
    fn from(response: Response) -> Self {
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
}
