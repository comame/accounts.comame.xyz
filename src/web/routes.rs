use http::hyper::RequestAsync;
use http::request::Method;
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};

use super::get_remote_addr::get_remote_addr;
use crate::data::openid_provider::OpenIDProvider;
use crate::web::handler;

pub async fn routes(hyper_request: HyperRequest<Body>) -> HyperResponse<Body> {
    let start_time = std::time::SystemTime::now();

    let remote_address = get_remote_addr(&hyper_request);
    let req = RequestAsync::from(hyper_request).get().await;

    let response = match (req.method, req.path.as_ref()) {
        (Method::Get, "/signin") => handler::signin::page("signin"),
        (Method::Get, "/reauthenticate") => handler::signin::page("reauthenticate"),
        (Method::Get, "/confirm") => handler::signin::page("confirm"),
        (Method::Post, "/signin/google") => handler::signin_google::handler(&req),
        (Method::Post, "/api/signin-password") => {
            handler::signin::sign_in_with_password(&req, &remote_address)
        }
        (Method::Post, "/api/signin-session") => {
            handler::signin::sign_in_with_session(&req, &remote_address)
        }
        (Method::Get, "/signout") => handler::signout::signout(&req),
        (Method::Get, "/authenticate") => handler::oidc_authentication_request::handler(&req),
        (Method::Post, "/authenticate") => handler::oidc_authentication_request::handler(&req),
        (Method::Post, "/code") => handler::oidc_code_request::handle(&req),
        (Method::Get, "/userinfo") => handler::oidc_userinfo_request::handle(&req),
        (Method::Post, "/userinfo") => handler::oidc_userinfo_request::handle(&req),
        (Method::Get, "/oidc-callback/google") => {
            handler::oidc_callback::handler(&req, OpenIDProvider::Google, &remote_address).await
        }
        (Method::Get, "/.well-known/openid-configuration") => handler::discovery::handle_config(),
        (Method::Get, "/certs") => handler::discovery::handle_certs(),
        _ => handler::static_file::handler(&req),
    };

    let result = response.into();

    let time = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();

    println!("REQ {} {} {time}", req.method, req.path);

    result
}
