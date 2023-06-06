use mysql::params;
use mysql::prelude::*;

use super::mysql::get_conn;
use crate::data::oidc_relying_party::RelyingParty;

pub fn find_relying_party_by_id(client_id: &str) -> Option<RelyingParty> {
    let values: Vec<(String, Option<String>, String)> = get_conn().unwrap().exec_map(
        "SELECT P.client_id, U.redirect_uri, P.hashed_client_secret FROM relying_parties P LEFT OUTER JOIN redirect_uris U ON P.client_id = U.client_id WHERE P.client_id = :id",
        params! { "id" => client_id },
        |pair: (String, mysql::Value, String)| {
            match pair.1 {
                mysql::Value::Bytes(bytes) => {
                    (pair.0, Some(String::from_utf8(bytes).unwrap()), pair.2)
                },
                mysql::Value::NULL => {
                    (pair.0, None, pair.2)
                },
                _ => { panic!() }
            }
        }
    ).unwrap();

    if values.is_empty() {
        return None;
    }

    let first = values.first().unwrap();
    let mut relying_party = RelyingParty {
        client_id: first.0.clone(),
        redirect_uris: vec![],
        hashed_client_secret: first.2.clone(),
    };

    for value in values {
        if let Some(uri) = value.1 {
            relying_party.redirect_uris.push(uri);
        }
    }

    Some(relying_party)
}

pub fn register_relying_party(client_id: &str, hashed_secret: &str) -> Result<(), ()> {
    let result = get_conn().unwrap().exec_drop(
        "INSERT INTO relying_parties VALUES (:id, :secret)",
        params! { "id" => client_id, "secret" => hashed_secret },
    );

    if result.is_err() {
        Err(())
    } else {
        Ok(())
    }
}

pub fn add_redirect_uri(client_id: &str, redirect_uri: &str) -> Result<(), ()> {
    // May fails because of unique constraint
    let result = get_conn().unwrap().exec_drop(
        "INSERT INTO redirect_uris (client_id, redirect_uri) VALUES (:id, :uri)",
        params! { "id" => client_id, "uri" => redirect_uri },
    );

    if result.is_ok() {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::oidc_relying_party::RelyingParty;
    use crate::db::_test_init::init_mysql;

    #[test]
    fn have_redirect_uri() {
        init_mysql();
        let client_id = "db_rp_have_redirect_uri.comame.dev";
        register_relying_party(client_id, "secret").unwrap();
        add_redirect_uri(client_id, "https://rp.comame.dev/redirect_1").unwrap();
        add_redirect_uri(client_id, "https://rp.comame.dev/redirect_2").unwrap();
        let rp = find_relying_party_by_id(client_id).unwrap();

        assert_eq!(
            rp,
            RelyingParty {
                client_id: client_id.to_string(),
                redirect_uris: vec!(
                    "https://rp.comame.dev/redirect_1".to_string(),
                    "https://rp.comame.dev/redirect_2".to_string()
                ),
                hashed_client_secret: "secret".to_string()
            }
        );
    }

    #[test]
    fn no_redirect_uri() {
        init_mysql();
        let client_id = "db_rp_no_redirect_uri.comame.dev";
        register_relying_party(client_id, "secret").unwrap();
        let rp = find_relying_party_by_id(client_id).unwrap();

        assert_eq!(
            rp,
            RelyingParty {
                client_id: client_id.to_string(),
                redirect_uris: vec!(),
                hashed_client_secret: "secret".to_string()
            }
        );
    }
}
