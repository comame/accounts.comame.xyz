use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct JwkKey {
    pub e: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub r#use: String,
    pub kty: String,
}

#[derive(Clone, Deserialize)]
pub struct Jwk {
    pub keys: Vec<JwkKey>,
}
