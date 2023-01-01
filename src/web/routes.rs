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
        (Method::Post, "/api/signin-session") => handler::signin::sign_in_with_session(&req),
        (Method::Get, "/signout") => handler::signout::signout(&req),
        (Method::Post, "/api/signin-continue") => {
            handler::signin_continue::handler(&req, &remote_address)
        }
        (Method::Post, "/api/signin-continue-nointeraction-fail") => {
            handler::signin_continue::no_interaction_fail(&req)
        }
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
        (Method::Post, "/tools/id-token") => handler::tools_id_token::handle(&req),
        (Method::Get, "/dash") => handler::dash::index(&req),
        (Method::Get, "/dash/signin") => handler::dash::signin(&req),
        (Method::Get, "/dash/callback") => handler::dash::callback(&req).await,
        (Method::Post, "/dash/rp/list") => handler::dash_relying_party::list_rp(&req),
        (Method::Post, "/dash/rp/create") => handler::dash_relying_party::create_rp(&req),
        (Method::Post, "/dash/rp/update_secret") => {
            handler::dash_relying_party::update_secret(&req)
        }
        (Method::Post, "/dash/rp/delete") => handler::dash_relying_party::delete_rp(&req),
        (Method::Post, "/dash/rp/redirect_uri/add") => {
            handler::dash_relying_party::add_redirect_uri(&req)
        }
        (Method::Post, "/dash/rp/redirect_uri/remove") => {
            handler::dash_relying_party::delete_redirect_uri(&req)
        }
        (Method::Post, "/dash/rp/binding/list") => {
            handler::dash_relying_party::list_user_binding(&req)
        }
        (Method::Post, "/dash/rp/binding/add") => {
            handler::dash_relying_party::add_user_binding(&req)
        }
        (Method::Post, "/dash/rp/binding/remove") => {
            handler::dash_relying_party::remove_user_binding(&req)
        }
        (Method::Post, "/dash/user/list") => handler::dash_user::list_user(&req),
        (Method::Post, "/dash/user/create") => handler::dash_user::create_user(&req),
        (Method::Post, "/dash/user/delete") => handler::dash_user::delete_user(&req),
        (Method::Post, "/dash/user/password/change") => handler::dash_user::insert_password(&req),
        (Method::Post, "/dash/user/password/remove") => handler::dash_user::remove_password(&req),
        (Method::Post, "/dash/user/session/list") => {
            todo!()
        }
        (Method::Post, "/dash/user/session/revoke") => {
            todo!()
        }
        (Method::Post, "/dash/user/authentication/list") => {
            handler::dash_user::list_token_issues(&req)
        }
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
