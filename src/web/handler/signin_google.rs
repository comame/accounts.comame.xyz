use http::cookies::{self};
use http::request::Request;
use http::response::Response;
use serde_json::from_str;

use crate::data::openid_provider::OpenIDProvider;
use crate::oidc::relying_party::generate_authentication_endpoint_url;
use crate::web::data::oidc_initiate_rp_request::OidcInitiateRpRequest;

fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.status = 403;
    res.body = Some(r#"{"error": "bad_request""#.into());
    res
}

fn response_redirect_browser(location: &str) -> Response {
    let mut res = Response::new();
    res.body = Some(format!(r#"{{"location": "{location}"}}"#));
    res
}

pub fn handler(req: &Request) -> Response {
    let body = req.body.clone().unwrap();
    let body = from_str::<OidcInitiateRpRequest>(&body);
    if let Err(_err) = body {
        return response_bad_request();
    }
    let body = body.unwrap();

    let result = generate_authentication_endpoint_url(
        &body.state_id,
        OpenIDProvider::Google,
        &body.user_agent_id,
    );

    let mut res = response_redirect_browser(&result.redirect_url);
    let cookie_value = cookies::build("rp", &result.state_id).build();
    res.cookies.push(cookie_value);

    res
}
