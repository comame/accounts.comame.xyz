use crate::data::authentication::LoginPrompt;
use crate::http::parse_form_urlencoded::parse;
use std::panic;

#[derive(Debug)]
pub struct AuthenticationRequest {
    pub scope: String,
    pub response_type: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub prompt: Option<LoginPrompt>,
    pub max_age: Option<u64>,
    pub id_token_hint: Option<String>,
}

impl AuthenticationRequest {
    pub fn parse_query(query: &str) -> Result<AuthenticationRequest, ()> {
        let map = parse(query)?;

        let scope = map.get("scope");
        let response_type = map.get("response_type");
        let client_id = map.get("client_id");
        let redirect_uri = map.get("redirect_uri");

        if scope.is_none()
            || response_type.is_none()
            || client_id.is_none()
            || redirect_uri.is_none()
        {
            return Err(());
        }

        let prompt = map.get("prompt");
        let mut prompt_parsed: Option<LoginPrompt> = None;
        if let Some(value) = prompt {
            let prompt = LoginPrompt::parse(value.as_str());
            if prompt.is_err() {
                return Err(());
            }
            prompt_parsed = Some(prompt.unwrap());
        }

        let max_age = map.get("max_age");
        let mut max_age_parsed: Option<u64> = None;
        if let Some(max_age) = max_age {
            let max_age = max_age.parse::<u64>();
            if max_age.is_err() {
                return Err(());
            }

            max_age_parsed = Some(max_age.unwrap());
        }

        Ok(Self {
            scope: scope.unwrap().clone(),
            response_type: response_type.unwrap().clone(),
            client_id: client_id.unwrap().clone(),
            redirect_uri: redirect_uri.unwrap().clone(),
            state: map.get("state").cloned(),
            nonce: map.get("nonce").cloned(),
            prompt: prompt_parsed,
            max_age: max_age_parsed,
            id_token_hint: map.get("id_token_hint").cloned(),
        })
    }
}
