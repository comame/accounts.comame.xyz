use hyper::{Body, Request};

pub fn get_remote_addr(req: &Request<Body>) -> String {
    let opt = req.headers().get("x-forwarded-for").cloned();
    match opt {
        Some(v) => v.to_str().unwrap().to_string(),
        None => String::from("0.0.0.0"),
    }
}
