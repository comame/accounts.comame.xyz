use hyper::{Body, Method, Request, Response, StatusCode};

use crate::http::mime_types::{extract_extension, get_mime_types};
use crate::http::set_header::set_header;
use crate::http::{handler, static_file};

pub async fn routes(req: Request<Body>) -> Response<Body> {
    let start_time = std::time::SystemTime::now();

    let mut response = Response::new(Body::empty());

    let uri = req.uri().clone();
    let method = req.method().clone();

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/signin") => {
            response = handler::signin::page("signin");
        }
        (&Method::GET, "/reauthenticate") => {
            response = handler::signin::page("reauthenticate");
        }
        (&Method::GET, "/confirm") => {
            response = handler::signin::page("confirm");
        }
        (&Method::GET, "/signout") => {
            response = handler::signout::signout(req).await;
        }
        (&Method::POST, "/api/signin-password") => {
            response = handler::signin::sign_in_with_password(req).await;
        }
        (&Method::POST, "/api/signin-session") => {
            response = handler::signin::sign_in_with_session(req).await;
        }
        (&Method::POST, "/api/signin-continue") => {
            response = handler::signin_continue::handler(req).await;
        }
        (&Method::POST, "/api/signin-continue-nointeraction-fail") => {
            response = handler::signin_continue::no_interaction_fail(req).await;
        }
        (&Method::GET, "/authenticate") => {
            response = handler::oidc_authentication_request::handler(req).await;
        }
        (&Method::POST, "/authenticate") => {
            response = handler::oidc_authentication_request::handler(req).await;
        }
        (&Method::POST, "/code") => {
            response = handler::oidc_code_request::handle(req).await;
        }
        (&Method::GET, "/.well-known/openid-configuration") => {
            response = handler::discovery::handle_config(req).await;
        }
        (&Method::GET, "/certs") => {
            response = handler::discovery::handle_certs(req).await;
        }
        (&Method::POST, "/tools/id-token") => {
            response = handler::tools_id_token::handle(req).await;
        }
        (&Method::POST, "/tools/session") => {
            response = handler::tools_session_inspect::handle(req).await;
        }
        (&Method::POST, "/tools/session-revoke") => {
            todo!()
        }
        (&Method::GET, "/rp/callback") => {
            response = handler::rp_callback::handler(req).await;
        }
        (&Method::GET, "/dash") => {
            response = handler::dash::index(req).await;
        }
        (&Method::GET, "/dash/signin") => {
            response = handler::dash::signin(req).await;
        }
        (&Method::GET, "/dash/callback") => {
            response = handler::dash::callback(req).await;
        }
        (&Method::POST, "/dash/rp/list") => {
            response = handler::dash_relying_party::list_rp(req).await;
        }
        (&Method::POST, "/dash/rp/create") => {
            response = handler::dash_relying_party::create_rp(req).await;
        }
        (&Method::POST, "/dash/rp/delete") => {
            response = handler::dash_relying_party::delete_rp(req).await;
        }
        (&Method::POST, "/dash/rp/redirect_uri/add") => {
            response = handler::dash_relying_party::add_redirect_uri(req).await;
        }
        (&Method::POST, "/dash/rp/redirect_uri/remove") => {
            response = handler::dash_relying_party::delete_redirect_uri(req).await;
        }
        (&Method::POST, "/dash/user/list") => {
            response = handler::dash_user::list_user(req).await;
        }
        (&Method::POST, "/dash/user/create") => {
            response = handler::dash_user::create_user(req).await;
        }
        (&Method::POST, "/dash/user/delete") => {
            response = handler::dash_user::delete_user(req).await;
        }
        (&Method::POST, "/dash/user/password/change") => {
            response = handler::dash_user::insert_password(req).await;
        }
        (&Method::POST, "/dash/user/password/remove") => {
            response = handler::dash_user::remove_password(req).await;
        }
        (&Method::POST, "/dash/user/session/list") => {
            todo!()
        }
        (&Method::POST, "/dash/user/session/revoke") => {
            todo!()
        }
        (&Method::POST, "/dash/user/authentication/list") => {
            todo!()
        }
        _ => {
            let file = static_file::read(req.uri().path());

            let uri = req.uri().to_string();
            let extension = extract_extension(&uri);
            let content_type = get_mime_types(&extension);

            if let Some(content_type) = content_type {
                set_header(&mut response, "Content-Type", &content_type);
            }

            if file.is_ok() {
                *response.body_mut() = Body::from(file.unwrap());
            } else {
                *response.body_mut() = "Not Found".into();
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
    };

    let time = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();

    println!("REQ {method} {} {time}", uri.path());

    response
}
