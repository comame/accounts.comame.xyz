use hyper::{Body, Response};

use crate::crypto::rand;
use crate::http::static_file;

pub fn handler() -> Response<Body> {
    let mut response = Response::new(Body::empty());

    let html_file_vec = static_file::read("/sign-in.html").unwrap();
    let html_file = String::from_utf8(html_file_vec).unwrap();

    let token = rand::random_str(32);

    let replaced_html_file = html_file.replace("$CSRF", token.as_str());

    *response.body_mut() = Body::from(replaced_html_file);

    response
}
