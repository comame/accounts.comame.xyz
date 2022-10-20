use std::env;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use super::authenticate_client::authenticate_client;
use crate::data::oidc_flow::id_token_claim::IdTokenClaim;

use crate::data::rsa_keypair::RsaKeypair;
use crate::time::now;

pub fn verify_id_token(
    client_id: &str,
    client_secret: &str,
    id_token: &str,
    nonce: Option<String>,
) -> Result<IdTokenClaim, ()> {
    authenticate_client(client_id, client_secret)?;

    let pubkey = RsaKeypair::get().public;

    let claim = decode::<IdTokenClaim>(
        id_token,
        &DecodingKey::from_rsa_pem(pubkey.as_bytes()).unwrap(),
        &Validation::new(Algorithm::RS256),
    );
    if claim.is_err() {
        return Err(());
    }
    let claim = claim.unwrap().claims;

    if client_id != claim.aud {
        return Err(());
    }

    let issuer = env::var("HOST").unwrap();
    if issuer != claim.iss {
        return Err(());
    }

    if claim.exp < now() {
        return Err(());
    }

    if let Some(nonce) = nonce {
        if let Some(ref nonce_claim) = claim.nonce {
            if nonce.as_str() != nonce_claim {
                return Err(());
            }
        }
    }

    Ok(claim)
}
