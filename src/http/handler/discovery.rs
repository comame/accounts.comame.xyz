use std::env;

use hyper::{Body, Request, Response};

use crate::data::rsa_keypair::RsaKeypair;
use crate::http::static_file;

pub async fn handle_config(_req: Request<Body>) -> Response<Body> {
    let file = static_file::read("/openid-config.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let replaced = file.replace("$HOST", &env::var("HOST").unwrap());

    Response::new(Body::from(replaced))
}

pub async fn handle_certs(_req: Request<Body>) -> Response<Body> {
    let file = static_file::read("/certs.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let keypair = RsaKeypair::get();

    let pubkey = keypair.public.replace("\n", "\\n");

    let json = file
        .replace("$N", &keypair.n())
        .replace("$E", &keypair.e())
        .replace("$KID", &keypair.kid)
        .replace("$PUBKEY", &pubkey);

    Response::new(Body::from(json))
}
