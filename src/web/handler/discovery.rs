use std::env;

use http::response::Response;

use crate::data::rsa_keypair::RsaKeypair;
use crate::web::static_file;

pub fn handle_config() -> Response {
    let file = static_file::read("/openid-config.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let replaced = file.replace("$HOST", &env::var("HOST").unwrap());

    let mut res = Response::new();
    res.body = Some(replaced);
    res
}

pub fn handle_certs() -> Response {
    let file = static_file::read("/certs.json").unwrap();
    let file = String::from_utf8(file).unwrap();

    let keypair = RsaKeypair::get();

    let pubkey = keypair.public.replace('\n', "\\n");

    let json = file
        .replace("$N", &keypair.n())
        .replace("$E", &keypair.e())
        .replace("$KID", &keypair.kid)
        .replace("$PUBKEY", &pubkey);

    let mut res = Response::new();
    res.body = Some(json);
    res
}
