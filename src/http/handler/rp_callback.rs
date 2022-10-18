use hyper::{Body, Request, Response};

use crate::http::static_file;

pub async fn handler(_req: Request<Body>) -> Response<Body> {
    let result = static_file::read("/callback.html");
    Response::new(Body::from(String::from_utf8(result.unwrap()).unwrap()))
}
