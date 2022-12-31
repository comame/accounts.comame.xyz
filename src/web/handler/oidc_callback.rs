use http::enc::form_urlencoded;
use http::request::Request;
use http::response::Response;

use crate::data::openid_provider::OpenIDProvider;
use crate::oidc::relying_party::callback;

fn response_bad_request(msg: &str) -> Response {
    let mut res = Response::new();
    res.status = 403;
    res.body = Some(format!(r#"{{error: "{}"}}"#, msg));
    res
}

pub async fn handler(req: &Request, op: OpenIDProvider) -> Response {
    let query = req.query.clone();
    if query.is_none() {
        return response_bad_request("bad_request");
    }
    let query = query.unwrap();
    let query = form_urlencoded::parse(&query);
    if query.is_err() {
        return response_bad_request("melformed_query_format");
    }
    let query = query.unwrap();

    let state = query.get("state").cloned();
    let code = query.get("code").cloned();

    if state.is_none() || code.is_none() {
        return response_bad_request("missing_parameters");
    }

    let state = state.unwrap();
    let code = code.unwrap();

    let result = callback(&state, &code, op).await;

    if let Err(_) = result {
        return response_bad_request("token_request_failed");
    }

    Response::new()
}
