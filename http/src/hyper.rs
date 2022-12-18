use std::borrow::BorrowMut;
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
            origin: None,
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

impl Response {
    async fn from_async(hyper_res: HyperResponse<HyperBody>) -> Self {
        let mut res = Response::new();
        res.status = hyper_res.status().as_u16();

        let mut cookies: Vec<String> = vec![];
        for key in hyper_res.headers().keys() {
            let value = hyper_res
                .headers()
                .get(key)
                .unwrap()
                .to_owned()
                .to_str()
                .unwrap()
                .to_string();
            res.headers
                .borrow_mut()
                .insert(normalize_header_key(&key.to_string()), value.clone());

            if normalize_header_key(&key.to_string()) == "Set-Cookie" {
                cookies.push(value);
            }
        }

        res.cookies = cookies;
        res.body = Some(parse_body(hyper_res.into_body()).await.unwrap());

        res
    }
}

pub struct RequestAsync {
    get: Pin<Box<dyn Future<Output = Request> + Send + 'static>>,
}

impl RequestAsync {
    pub async fn get(self) -> Request {
        join!(self.get).0
    }
}

pub struct ResponseAsync {
    get: Pin<Box<dyn Future<Output = Response> + Send + 'static>>,
}

impl ResponseAsync {
    pub async fn get(self) -> Response {
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

impl From<HyperResponse<HyperBody>> for ResponseAsync {
    fn from(hyper_res: HyperResponse<HyperBody>) -> Self {
        Self {
            get: Box::pin(Response::from_async(hyper_res)),
        }
    }
}

impl TryFrom<Request> for HyperRequest<HyperBody> {
    type Error = ();

    fn try_from(req: Request) -> Result<Self, Self::Error> {
        let mut hyper_req = HyperRequest::new(HyperBody::empty());

        // TODO: まともな URL パーサを作って検証する

        if req.origin.is_none() {
            return Err(());
        }

        let url = if let Some(query) = req.query {
            format!("{}{}?{query}", req.origin.unwrap(), req.path)
        } else {
            format!("{}{}", req.origin.unwrap(), req.path)
        };
        *hyper_req.uri_mut() = url.parse().unwrap();

        *hyper_req.method_mut() = match req.method {
            Method::Get => HyperMethod::GET,
            Method::Post => HyperMethod::POST,
        };

        for (key, value) in req.headers {
            let header_key = HeaderName::from_str(&key).unwrap();
            let header_value = HeaderValue::from_str(&value).unwrap();
            hyper_req.headers_mut().append(header_key, header_value);
        }

        let cookie_value = req
            .cookies
            .into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<String>>()
            .join("; ");
        let cookie_value = HeaderValue::from_str(&cookie_value).unwrap();
        hyper_req.headers_mut().append("Cookie", cookie_value);

        if let Some(body) = req.body {
            *hyper_req.body_mut() = HyperBody::from(body);
        }

        Ok(hyper_req)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_hyper_request() {
        let hyper_request = HyperRequest::builder()
            .method(HyperMethod::POST)
            .uri("https://example.com/foo?key=value&bar=baz")
            .header("Foo", "bar")
            .header("content-type", "text/plain")
            .header("Cookie", "key=value")
            .body(HyperBody::from("body"))
            .unwrap();
        let request = RequestAsync::from(hyper_request).get().await;

        assert_eq!(request.method, Method::Post);
        assert_eq!(request.path, "/foo");
        assert_eq!(request.query.unwrap(), "key=value&bar=baz");
        assert_eq!(request.body.unwrap(), "body");

        let headers = request.headers;
        assert_eq!(headers.len(), 3);
        assert_eq!(headers.get("Foo").unwrap(), "bar");
        assert_eq!(headers.get("Cookie").unwrap(), "key=value");
        assert_eq!(headers.get("Content-Type").unwrap(), "text/plain");

        let cookies = request.cookies;
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies.get("key").unwrap(), "value");
    }

    #[tokio::test]
    async fn from_hyper_response() {
        let hyper_response = HyperResponse::builder()
            .status(StatusCode::OK)
            .header("Set-Cookie", "key1=value1; Secure; HttpOnly")
            .header("Foo", "bar")
            .body(HyperBody::from("hello"))
            .unwrap();
        let response = ResponseAsync::from(hyper_response).get().await;

        assert_eq!(response.status, 200);
        assert_eq!(response.body.unwrap(), "hello");

        let headers = response.headers;
        assert_eq!(headers.len(), 2);
        assert_eq!(headers.get("Foo").unwrap(), "bar");

        let cookies = response.cookies;
        assert_eq!(cookies, vec!["key1=value1; Secure; HttpOnly",]);
    }

    #[tokio::test]
    async fn to_hyper_request() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());

        let mut cookies = HashMap::new();
        cookies.insert("key".to_string(), "value".to_string());

        let request = Request {
            origin: Some("https://example.com".to_string()),
            method: Method::Post,
            path: "/foo/bar".to_string(),
            query: Some("foo=bar".to_string()),
            headers,
            cookies,
            body: Some("Hello, world!".to_string()),
        };
        let hyper_request: HyperRequest<HyperBody> = request.try_into().unwrap();

        assert_eq!(hyper_request.method(), HyperMethod::POST);
        assert_eq!(
            hyper_request.uri().to_string(),
            "https://example.com/foo/bar?foo=bar"
        );

        let headers = hyper_request.headers();
        assert_eq!(headers.len(), 2);
        assert_eq!(
            headers.get("Content-Type").unwrap().to_str().unwrap(),
            "text/plain"
        );
        assert_eq!(
            headers.get("Cookie").unwrap().to_str().unwrap(),
            "key=value"
        );

        let body = parse_body(hyper_request.into_body()).await.unwrap();
        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn to_hyper_response() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());

        let response = Response {
            status: 200,
            headers,
            cookies: vec!["key=value; HttpOnly; Secure".to_string()],
            body: Some("Hello, world!".to_string()),
        };

        let hyper_response: HyperResponse<HyperBody> = response.into();

        assert_eq!(hyper_response.status(), StatusCode::OK);

        let headers = hyper_response.headers();
        assert_eq!(headers.len(), 2);
        assert_eq!(
            headers.get("content-type").unwrap().to_str().unwrap(),
            "text/plain"
        );
        assert_eq!(
            headers.get("set-cookie").unwrap().to_str().unwrap(),
            "key=value; HttpOnly; Secure"
        );
    }
}
