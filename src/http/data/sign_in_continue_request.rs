use crate::http::parse_form_urlencoded::parse;

pub struct SignInContinueRequest {
    pub csrf_token: String,
}

impl SignInContinueRequest {
    pub fn parse_from(str: &str) -> Result<Self, ()> {
        let map = parse(str)?;
        let token = map.get("csrf_token");

        if token.is_none() {
            return Err(());
        }

        let token = token.unwrap().clone();

        Ok(Self { csrf_token: token })
    }
}
