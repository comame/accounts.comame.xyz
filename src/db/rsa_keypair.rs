use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::rsa_keypair::RsaKeypair;

pub fn insert_ignore(keypair: &RsaKeypair) {
    get_conn().unwrap().exec_drop(
        "INSERT IGNORE INTO rsa_keypair (public, private, kid) VALUES (:pub, :priv, :kid) ON DUPLICATE KEY UPDATE public=:pub, private=:priv",
        params! { "pub" => keypair.public.clone(), "priv" => keypair.private.clone(), "kid" => keypair.kid.clone() }
    ).unwrap();
}

pub fn insert_force(keypair: &RsaKeypair) {
    get_conn().unwrap().exec_drop(
        "INSERT INTO rsa_keypair (public, private, kid) VALUES (:pub, :priv, :kid) ON DUPLICATE KEY UPDATE public=:pub, private=:priv",
        params! { "pub" => keypair.public.clone(), "priv" => keypair.private.clone(), "kid" => keypair.kid.clone() }
    ).unwrap();
}

/// 必ず存在することを仮定
pub fn get() -> RsaKeypair {
    let result = get_conn()
        .unwrap()
        .query_map(
            "SELECT public, private, kid FROM rsa_keypair",
            |(public, private, kid)| RsaKeypair {
                public,
                private,
                kid,
            },
        )
        .unwrap();

    result.first().expect("RsaKeypair is not set.").to_owned()
}
