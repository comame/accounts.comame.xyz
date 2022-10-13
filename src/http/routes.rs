use hyper::{Body, Method, Request, Response, StatusCode};

use crate::http::{handler, static_file};

pub async fn routes(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    println!("Request {}", req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/signin") => {
            response = handler::signin::page();
        }
        (&Method::GET, "/reauthenticate") => {
            todo!();
        }
        (&Method::GET, "/confirm") => {
            todo!();
        }
        (&Method::POST, "/signin-password") => {
            response = handler::signin::sign_in_with_password(req).await;
        }
        (&Method::POST, "/signin-session") => {
            response = handler::signin::sign_in_with_session(req).await;
        }
        (&Method::GET, "/signout") => {
            response = handler::signout::signout(req).await;
        }
        (&Method::POST, "/signin-continue") => {
            response = handler::signin_continue::handler(req).await;
        }
        (&Method::POST, "/signin-continue-nointeraction-fail") => {
            response = handler::signin_continue::no_interaction_fail(req).await;
        }
        (&Method::GET, "/authenticate") => {
            response = handler::oidc_authentication_request::handler(req).await;
        }
        (&Method::POST, "/authenticate") => {
            response = handler::oidc_authentication_request::handler(req).await;
        }
        _ => {
            let file = static_file::read(req.uri().path());

            if file.is_ok() {
                *response.body_mut() = Body::from(file.unwrap());
            } else {
                *response.body_mut() = "Not Found".into();
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
    };

    response
}
