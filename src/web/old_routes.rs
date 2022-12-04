use hyper::{Body, Method, Request, Response, StatusCode};

use crate::web::cachable_file::CacheResult;
use crate::web::mime_types::{extract_extension, get_mime_types};
use crate::web::set_header::{set_header, set_no_store_old};
use crate::web::{cachable_file, old_handler, static_file};

#[deprecated]
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
        (&Method::POST, "/api/signin-password") => {
            set_no_store_old(&mut response);
            response = old_handler::signin::sign_in_with_password(req).await;
        }
        (&Method::POST, "/api/signin-session") => {
            set_no_store_old(&mut response);
            response = old_handler::signin::sign_in_with_session(req).await;
        }
        (&Method::POST, "/api/signin-continue") => {
            set_no_store_old(&mut response);
            response = old_handler::signin_continue::handler(req).await;
        }
        (&Method::POST, "/api/signin-continue-nointeraction-fail") => {
            set_no_store_old(&mut response);
            response = old_handler::signin_continue::no_interaction_fail(req).await;
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
