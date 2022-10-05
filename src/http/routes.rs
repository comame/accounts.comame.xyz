use hyper::{Body, Method, Request, Response, StatusCode};

use crate::http::handler;
use crate::http::static_file;

pub async fn routes(req: Request<Body>) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    println!("Request {}", req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/signin") => {
            response = handler::signin::page();
        }
        (&Method::POST, "/signin-password") => {
            response = handler::signin::sign_in_with_password(req).await;
        }
        (&Method::POST, "/signin-session") => {
            response = handler::signin::sign_in_with_session(req).await;
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
