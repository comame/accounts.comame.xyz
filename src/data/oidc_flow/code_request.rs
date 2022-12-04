use std::collections::HashMap;

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

impl CodeRequest {
    pub fn parse(str: &str, id: Option<&str>, secret: Option<&str>) -> Result<Self, ()> {
        let map = http::enc::form_urlencoded::parse(str)?;

        let grant_type = get_or_result(&map, "grant_type")?.clone();
        let code = get_or_result(&map, "code")?.clone();
        let redirect_uri = get_or_result(&map, "redirect_uri")?.clone();
        let client_id = if id.is_some() {
            id.unwrap().to_string()
        } else {
            get_or_result(&map, "client_id")?.clone()
        };
        let client_secret = if secret.is_some() {
            Some(secret.unwrap().to_string())
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
