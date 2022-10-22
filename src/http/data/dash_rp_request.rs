use serde::Deserialize;

#[derive(Deserialize)]
pub struct RelyingPartyClientIdRequest {
    pub token: String,
    pub client_id: String,
}

#[derive(Deserialize)]
pub struct RelyingPartyAddRedirectUriRequest {
    pub token: String,
    pub client_id: String,
    pub redirect_uri: String,
}
