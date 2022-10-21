use serde::Deserialize;

#[derive(Deserialize)]
pub struct RelyingPartyClientIdRequest {
    pub client_id: String,
}
