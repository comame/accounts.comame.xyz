use serde::Deserialize;

#[derive(Deserialize)]
pub struct StandardRequest {
    pub token: String,
}
