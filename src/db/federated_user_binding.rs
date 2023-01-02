use mysql::params;
use mysql::prelude::*;

use crate::data::federated_user_binding::FederatedUserBinding;
use crate::data::openid_provider::OpenIDProvider;

use super::mysql::get_conn;

pub fn exists_federated_user_binding(relying_party_id: &str, issuer: &str) -> bool {
    let result = get_conn().unwrap()
        .exec_map(
            "SELECT COUNT(*) FROM federated_user_binding WHERE relying_party_id=:rp AND issuer = :issuer",
            params! {
                "rp" => relying_party_id,
                "issuer" => issuer
            },
            |(count,): (usize,)| count,
        ).unwrap();
    result.first().cloned().unwrap() != 0
}

pub fn insert_federated_user_binding(relying_party_id: &str, issuer: &str) -> Result<(), ()> {
    let result = get_conn().unwrap().exec_drop(
        "INSERT INTO federated_user_binding (relying_party_id, issuer) VALUES (:rp, :iss)",
        params! {
            "rp" => relying_party_id,
            "iss" => issuer
        },
    );

    if let Err(_) = result {
        Err(())
    } else {
        Ok(())
    }
}

pub fn delete_federate_user_binding(relying_party_id: &str, issuer: &str) -> Result<(), ()> {
    let result = get_conn().unwrap().exec_drop(
        "DELETE FROM federated_user_binding WHERE relying_party_id = :rp AND issuer = :iss",
        params! {
            "rp" => relying_party_id,
            "iss" => issuer,
        },
    );

    if let Err(_) = result {
        Err(())
    } else {
        Ok(())
    }
}

pub fn list_federated_user_binding(
    relying_party_id: &str,
) -> Result<Vec<FederatedUserBinding>, ()> {
    let result: Result<Vec<FederatedUserBinding>, mysql::Error> = get_conn().unwrap().exec_map(
        "SELECT relying_party_id, issuer FROM federated_user_binding WHERE relying_party_id = :rp",
        params! {
            "rp" => relying_party_id,
        },
        |(relying_party_id, issuer): (String, String)| FederatedUserBinding {
            relying_party_id: relying_party_id,
            issuer: OpenIDProvider::parse(&issuer).unwrap(),
        },
    );
    if let Ok(v) = result {
        Ok(v)
    } else {
        Err(())
    }
}
