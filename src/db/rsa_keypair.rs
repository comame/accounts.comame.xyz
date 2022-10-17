use mysql::{prelude::*, params};

use crate::data::rsa_keypair::RsaKeypair;

use super::mysql::get_conn;

pub fn insert_ignore(keypair: &RsaKeypair) {
    get_conn().unwrap().exec_drop(
        "INSERT IGNORE INTO rsa_keypair (public, private) VALUES (:pub, :priv) ON DUPLICATE KEY UPDATE public=:pub, private=:priv",
        params! { "pub" => keypair.public.clone(), "priv" => keypair.private.clone() }
    ).unwrap();
}

pub fn insert_force(keypair: &RsaKeypair) {
    get_conn().unwrap().exec_drop(
        "INSERT INTO rsa_keypair (public, private) VALUES (:pub, :priv) ON DUPLICATE KEY UPDATE public=:pub, private=:priv",
        params! { "pub" => keypair.public.clone(), "priv" => keypair.private.clone() }
    ).unwrap();
}

/// 必ず存在することを仮定
pub fn get() -> RsaKeypair {
    let result = get_conn().unwrap().query_map(
        "SELECT public, private FROM rsa_keypair",
        |(public, private)| RsaKeypair { public, private }
    ).unwrap();

    result.first().expect("RsaKeypair is not set.").to_owned()
}
