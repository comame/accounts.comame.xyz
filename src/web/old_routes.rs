use hyper::{Body, Method, Request, Response, StatusCode};

use crate::web::cachable_file::CacheResult;
use crate::web::mime_types::{extract_extension, get_mime_types};
use crate::web::set_header::{set_header, set_no_store};
use crate::web::{cachable_file, old_handler, static_file};

pub async fn routes(req: Request<Body>) -> Response<Body> {
    let start_time = std::time::SystemTime::now();

    let mut response = Response::new(Body::empty());

    let uri = req.uri().clone();
    let method = req.method().clone();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/signin") => {
            response = old_handler::signin::page("signin");
        }
        (&Method::GET, "/reauthenticate") => {
            response = old_handler::signin::page("reauthenticate");
        }
        (&Method::GET, "/confirm") => {
            response = old_handler::signin::page("confirm");
        }
        (&Method::GET, "/signout") => {
            response = old_handler::signout::signout(req).await;
        }
        (&Method::POST, "/api/signin-password") => {
            set_no_store(&mut response);
            response = old_handler::signin::sign_in_with_password(req).await;
        }
        (&Method::POST, "/api/signin-session") => {
            set_no_store(&mut response);
            response = old_handler::signin::sign_in_with_session(req).await;
        }
        (&Method::POST, "/api/signin-continue") => {
            set_no_store(&mut response);
            response = old_handler::signin_continue::handler(req).await;
        }
        (&Method::POST, "/api/signin-continue-nointeraction-fail") => {
            set_no_store(&mut response);
            response = old_handler::signin_continue::no_interaction_fail(req).await;
        }
        (&Method::GET, "/authenticate") => {
            set_no_store(&mut response);
            response = old_handler::oidc_authentication_request::handler(req).await;
        }
        (&Method::POST, "/authenticate") => {
            set_no_store(&mut response);
            response = old_handler::oidc_authentication_request::handler(req).await;
        }
        (&Method::POST, "/code") => {
            set_no_store(&mut response);
            response = old_handler::oidc_code_request::handle(req).await;
        }
        (&Method::GET, "/userinfo") => {
            set_no_store(&mut response);
            response = old_handler::oidc_userinfo_request::handle(req).await;
        }
        (&Method::POST, "/userinfo") => {
            set_no_store(&mut response);
            response = old_handler::oidc_userinfo_request::handle(req).await;
        }
        (&Method::GET, "/.well-known/openid-configuration") => {
            response = old_handler::discovery::handle_config(req).await;
        }
        (&Method::GET, "/certs") => {
            response = old_handler::discovery::handle_certs(req).await;
        }
        (&Method::POST, "/tools/id-token") => {
            response = old_handler::tools_id_token::handle(req).await;
        }
        (&Method::POST, "/tools/session-revoke") => {
            todo!()
        }
        (&Method::GET, "/dash") => {
            response = old_handler::dash::index(req).await;
        }
        (&Method::GET, "/dash/signin") => {
            response = old_handler::dash::signin(req).await;
        }
        (&Method::GET, "/dash/callback") => {
            response = old_handler::dash::callback(req).await;
        }
        (&Method::POST, "/dash/rp/list") => {
            response = old_handler::dash_relying_party::list_rp(req).await;
        }
        (&Method::POST, "/dash/rp/create") => {
            response = old_handler::dash_relying_party::create_rp(req).await;
        }
        (&Method::POST, "/dash/rp/update_secret") => {
            response = old_handler::dash_relying_party::update_secret(req).await;
        }
        (&Method::POST, "/dash/rp/delete") => {
            response = old_handler::dash_relying_party::delete_rp(req).await;
        }
        (&Method::POST, "/dash/rp/redirect_uri/add") => {
            response = old_handler::dash_relying_party::add_redirect_uri(req).await;
        }
        (&Method::POST, "/dash/rp/redirect_uri/remove") => {
            response = old_handler::dash_relying_party::delete_redirect_uri(req).await;
        }
        (&Method::POST, "/dash/rp/binding/list") => {
            response = old_handler::dash_relying_party::list_user_binding(req).await;
        }
        (&Method::POST, "/dash/rp/binding/add") => {
            response = old_handler::dash_relying_party::add_user_binding(req).await;
        }
        (&Method::POST, "/dash/rp/binding/remove") => {
            response = old_handler::dash_relying_party::remove_user_binding(req).await;
        }
        (&Method::POST, "/dash/user/list") => {
            response = old_handler::dash_user::list_user(req).await;
        }
        (&Method::POST, "/dash/user/create") => {
            response = old_handler::dash_user::create_user(req).await;
        }
        (&Method::POST, "/dash/user/delete") => {
            response = old_handler::dash_user::delete_user(req).await;
        }
        (&Method::POST, "/dash/user/password/change") => {
            response = old_handler::dash_user::insert_password(req).await;
        }
        (&Method::POST, "/dash/user/password/remove") => {
            response = old_handler::dash_user::remove_password(req).await;
        }
        (&Method::POST, "/dash/user/session/list") => {
            todo!()
        }
        (&Method::POST, "/dash/user/session/revoke") => {
            todo!()
        }
        (&Method::POST, "/dash/user/authentication/list") => {
            response = old_handler::dash_user::list_token_issues(req).await;
        }
        _ => {
            let file = cachable_file::read_with_etag(req.uri().path());

            let uri = req.uri().path().to_string();
            let extension = extract_extension(&uri);
            let content_type = get_mime_types(&extension);

            if let Some(content_type) = content_type {
                set_header(&mut response, "Content-Type", &content_type);
            }

            if file.is_none() {
                *response.body_mut() = "Not Found".into();
                *response.status_mut() = StatusCode::NOT_FOUND;
            } else {
                let file = file.unwrap();

                set_header(&mut response, "Cache-Control", "no-cache");

                match file {
                    CacheResult::Etag(etag) => {
                        let previous_etag = req.headers().get("If-None-Match").cloned();
                        if !cfg!(not(debug_assertions)) || previous_etag.is_none() {
                            set_header(&mut response, "Etag", &format!(r#""{}""#, etag));
                            *response.body_mut() = Body::from(static_file::read(&uri).unwrap());
                        } else {
                            let previous_etag = previous_etag.unwrap();
                            let previous_etag = previous_etag.to_str().unwrap();
                            if previous_etag == format!("\"{}\"", etag) {
                                *response.status_mut() = StatusCode::NOT_MODIFIED;
                            } else {
                                set_header(&mut response, "Etag", &format!(r#""{}""#, etag));
                                *response.body_mut() = Body::from(static_file::read(&uri).unwrap());
                            }
                        }
                    }
                    CacheResult::Value(value) => {
                        set_header(&mut response, "Etag", &format!(r#""{}""#, value.etag));
                        *response.body_mut() = Body::from(value.value);
                    }
                }
            }
        }
    };

    if !uri.clone().path().starts_with("/dev/") {
        set_header(
            &mut response,
            "Content-Security-Policy",
            "default-src 'self'; style-src 'self' 'unsafe-inline'",
        );
    }

    let time = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();

    println!("REQ {method} {} {time}", uri.path());

    response
}
