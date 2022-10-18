use hyper::{Body, Request, Response};
use std::env;

use crate::{data::rsa_keypair::RsaKeypair, http::static_file};

pub async fn handle_config(req: Request<Body>) -> Response<Body> {
    let file = static_file::read("/openid-config.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let host = env::var("HOST").unwrap();

    let replaced = file.replace("$HOST", &host);

    Response::new(Body::from(replaced))
}

pub async fn handle_certs(req: Request<Body>) -> Response<Body> {
    let file = static_file::read("/certs.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let keypair = RsaKeypair::get();

    let json = file
        .replace("$N", &keypair.n())
        .replace("$E", &keypair.e())
        .replace("$KID", &keypair.kid);

    Response::new(Body::from(json))
}
