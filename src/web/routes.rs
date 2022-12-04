use http::{
    hyper::{from_hyper_request, from_hyper_request_without_body, to_hyper_response},
    request::Method,
};
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};

use crate::web::handler;

use super::get_remote_addr::get_remote_addr;

pub async fn routes(hyper_request: HyperRequest<Body>) -> HyperResponse<Body> {
    let remote_address = get_remote_addr(&hyper_request);
    let req = from_hyper_request_without_body(&hyper_request);

    let response = match (req.method, req.path.as_ref()) {
        (Method::Get, "/signin") => handler::signin::page("signin"),
        (Method::Get, "/reauthenticate") => handler::signin::page("reauthenticate"),
        (Method::Get, "/confirm") => handler::signin::page("confirm"),
        (Method::Post, "/api/signin-password") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::signin::sign_in_with_password(req, &remote_address)
        }
        (Method::Post, "/api/signin-session") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::signin::sign_in_with_session(req)
        }
        (Method::Get, "/signout") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::signout::signout(req)
        }
        (Method::Post, "/api/signin-continue") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::signin_continue::handler(req, &remote_address)
        }
        (Method::Post, "/api/signin-continue-nointeraction-fail") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::signin_continue::no_interaction_fail(req)
        }
        (Method::Get, "/authenticate") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_authentication_request::handler(req)
        }
        (Method::Post, "/authenticate") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_authentication_request::handler(req)
        }
        (Method::Post, "/code") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_code_request::handle(req)
        }
        (Method::Get, "/userinfo") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_userinfo_request::handle(req)
        }
        (Method::Post, "/userinfo") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::oidc_userinfo_request::handle(req)
        }
        (Method::Get, "/.well-known/openid-configuration") => handler::discovery::handle_config(),
        (Method::Get, "/certs") => handler::discovery::handle_certs(),
        (Method::Get, "/dash") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash::index(req)
        }
        (Method::Get, "/dash/signin") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash::signin(req)
        }
        (Method::Get, "/dash/callback") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash::callback(req).await
        }
        (Method::Post, "/dash/rp/list") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::list_rp(req)
        }
        (Method::Post, "/dash/rp/create") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::create_rp(req)
        }
        (Method::Post, "/dash/rp/update_secret") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::update_secret(req)
        }
        (Method::Post, "/dash/rp/delete") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::delete_rp(req)
        }
        (Method::Post, "/dash/rp/redirect_uri/add") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::add_redirect_uri(req)
        }
        (Method::Post, "/dash/rp/redirect_uri/remove") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::delete_redirect_uri(req)
        }
        (Method::Post, "/dash/rp/binding/list") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::list_user_binding(req)
        }
        (Method::Post, "/dash/rp/binding/add") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::add_user_binding(req)
        }
        (Method::Post, "/dash/rp/binding/remove") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_relying_party::remove_user_binding(req)
        }
        (Method::Post, "/dash/user/list") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::list_user(req)
        }
        (Method::Post, "/dash/user/create") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::create_user(req)
        }
        (Method::Post, "/dash/user/delete") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::delete_user(req)
        }
        (Method::Post, "/dash/user/password/change") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::insert_password(req)
        }
        (Method::Post, "/dash/user/password/remove") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::remove_password(req)
        }
        (Method::Post, "/dash/user/session/list") => {
            todo!()
        }
        (Method::Post, "/dash/user/session/revoke") => {
            todo!()
        }
        (Method::Post, "/dash/user/authentication/list") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::dash_user::list_token_issues(req)
        }
        _ => return crate::web::old_routes::routes(hyper_request).await,
    };

    to_hyper_response(response)
}
