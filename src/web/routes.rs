use http::{
    hyper::{from_hyper_request, from_hyper_request_without_body, to_hyper_response},
    request::Method,
    response::Response,
};
use hyper::{Body, Request as HyperRequest, Response as HyperResponse};

use crate::web::handler;

use super::{
    cachable_file::{self, CacheResult},
    get_remote_addr::get_remote_addr,
    mime_types::{extract_extension, get_mime_types},
    static_file,
};

pub async fn routes(hyper_request: HyperRequest<Body>) -> HyperResponse<Body> {
    let start_time = std::time::SystemTime::now();

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
        (Method::Post, "/tools/id-token") => {
            let req = from_hyper_request(hyper_request).await; // FIXME: 移行後に消す
            handler::tools_id_token::handle(req)
        }
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
        _ => {
            let file = cachable_file::read_with_etag(&req.path);

            let extension = extract_extension(&req.path);
            let content_type = get_mime_types(&extension);

            let mut res = Response::new();

            if let Some(content_type) = content_type {
                res.headers.insert("Content-Type".to_string(), content_type);
            }

            if file.is_none() {
                res.body = Some("Not Found".to_string());
                res.status = 404;
            } else {
                let file = file.unwrap();

                res.headers
                    .insert("Cache-Control".to_string(), "no-cache".to_string());

                match file {
                    CacheResult::Etag(etag) => {
                        let previous_etag = req.headers.get("If-None-Match").cloned();
                        if !cfg!(not(debug_assertions)) || previous_etag.is_none() {
                            res.headers.insert("Etag".into(), format!(r#""{}""#, etag));
                            res.body = Some(
                                String::from_utf8(static_file::read(&req.path).unwrap()).unwrap(),
                            );
                        } else {
                            let previous_etag = previous_etag.unwrap();
                            if previous_etag == format!("\"{}\"", etag) {
                                res.status = 304; // NOT MODIFIED
                            } else {
                                res.headers.insert("Etag".into(), format!(r#""{}""#, etag));
                                res.body = Some(
                                    String::from_utf8(static_file::read(&req.path).unwrap())
                                        .unwrap(),
                                );
                            }
                        }
                    }
                    CacheResult::Value(value) => {
                        res.headers
                            .insert("Etag".into(), format!(r#""{}""#, value.etag));
                        res.body =
                            Some(String::from_utf8(static_file::read(&req.path).unwrap()).unwrap());
                    }
                }
            }
            res
        }
    };

    let result = to_hyper_response(response);

    let time = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();

    println!("REQ {} {} {time}", req.method, req.path);

    result
}
