use mysql::{params, prelude::*};

use crate::{
    data::{
        access_token::{AccessToken, ACCESS_TOKEN_EXPIRES_IN},
        oidc_flow::oidc_scope::Scopes,
    },
    time::{now, unixtime_to_datetime},
};

use super::mysql::get_conn;

pub fn insert_access_token(access_token: &AccessToken) {
    get_conn().unwrap().exec_drop(
        "INSERT INTO access_tokens (sub, scopes, token, created_at) VALUES (:sub, :scope, :token, :created_at)"
        , params! {
            "sub" => access_token.sub.to_string(),
            "scope" => access_token.scopes.to_string(),
            "token" => access_token.token.to_string(),
            "created_at" => unixtime_to_datetime(now()),
        }).unwrap();
}

pub fn get_access_token(token: &str) -> Option<AccessToken> {
    let now = unixtime_to_datetime(now());
    let result: Vec<(String, String, String)> = get_conn()
        .unwrap()
        .exec_map(
            "SELECT sub, scopes, token FROM access_tokens WHERE TIMESTAMPDIFF(SECOND, created_at, :now) < :expire AND token = :token",
            params! {
                "now" => now.to_string(),
                "token" => token.to_string(),
                "expire" => ACCESS_TOKEN_EXPIRES_IN,
            },
            |(sub, scopes, token)| (sub, scopes, token),
        )
        .unwrap();

    let first = result.first().cloned()?;

    Some(AccessToken {
        sub: first.0,
        scopes: Scopes::parse(&first.1),
        token: first.2,
        expires_in: ACCESS_TOKEN_EXPIRES_IN,
    })
}
