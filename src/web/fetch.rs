use http::{hyper::ResponseAsync, request::Request, response::Response};
use hyper::{Client, Request as HyperRequest};
use hyper_tls::HttpsConnector;

pub async fn fetch(request: &Request) -> Result<Response, ()> {
    let hyper_request = HyperRequest::try_from(request.clone());
    if let Err(_err) = hyper_request {
        return Err(());
    }

    let hyper_request = hyper_request.unwrap();
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let hyper_response = client.request(hyper_request).await;
    if let Err(err) = hyper_response {
        eprintln!("{}", err);
        return Err(());
    }

    let response = ResponseAsync::from(hyper_response.unwrap()).get().await;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::request::{Method, Request};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test() {
        let req = Request {
            origin: Some("https://example.com".into()),
            method: Method::Get,
            path: "/".into(),
            query: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: None,
        };
        let res = fetch(&req).await;
        let res = res.unwrap();
        assert_eq!(res.status, 200);
    }
}
