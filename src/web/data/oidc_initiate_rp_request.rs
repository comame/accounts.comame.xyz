use serde::Deserialize;

#[derive(Deserialize)]
pub struct OidcInitiateRpRequest {
    pub state_id: String,
    pub user_agent_id: String,
}
