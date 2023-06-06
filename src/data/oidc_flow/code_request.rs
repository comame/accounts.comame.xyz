use std::collections::HashMap;
use std::fmt::Display;

use http::query_builder::QueryBuilder;

#[derive(Debug)]
pub struct CodeRequest {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: Option<String>,
}

fn get_or_result<'a>(hash_map: &'a HashMap<String, String>, key: &str) -> Result<&'a String, ()> {
    match hash_map.get(key) {
        Some(v) => Ok(v),
        None => Err(()),
    }
}

impl Display for CodeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut txt = QueryBuilder::new();
        let txt = txt
            .append("grant_type", &self.grant_type)
            .append("code", &self.code)
            .append("redirect_uri", &self.redirect_uri)
            .append("client_id", &self.client_id);
        if let Some(secret) = &self.client_secret {
            txt.append("client_secret", secret);
        }
        write!(f, "{}", txt.build())
    }
}

impl CodeRequest {
    pub fn parse(str: &str, id: Option<&str>, secret: Option<&str>) -> Result<Self, ()> {
        let map = http::enc::form_urlencoded::parse(str)?;

        let grant_type = get_or_result(&map, "grant_type")?.clone();
        let code = get_or_result(&map, "code")?.clone();
        let redirect_uri = get_or_result(&map, "redirect_uri")?.clone();

        let client_id = if let Some(id) = id {
            id.to_string()
        } else {
            get_or_result(&map, "client_id")?.clone()
        };

        let client_secret = if let Some(secret) = secret {
            Some(secret.to_string())
        } else {
            map.get("client_secret").cloned()
        };

        Ok(Self {
            grant_type,
            code,
            redirect_uri,
            client_id,
            client_secret,
        })
    }
}
