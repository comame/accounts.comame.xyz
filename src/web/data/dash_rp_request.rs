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

#[derive(Deserialize)]
pub struct RelyingPartyBindingRequest {
    pub token: String,
    pub client_id: String,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct RelyingPartyFederatedUserBindingRequest {
    pub token: String,
    pub client_id: String,
    pub issuer: String,
}
