use serde::Serialize;

use super::oidc_flow::id_token_claim::IdTokenClaim;
use crate::db::idtoken_issues::{insert, list_by_sub};

#[derive(Serialize)]
pub struct IdTokenIssue {
    pub sub: String,
    pub aud: String,
    pub iat: u64,
}

impl IdTokenIssue {
    pub fn log(claim: &IdTokenClaim) {
        insert(&Self {
            sub: claim.sub.clone(),
            aud: claim.aud.clone(),
            iat: claim.iat,
        });
    }

    pub fn list_by_sub(subject: &str) -> Vec<Self> {
        list_by_sub(subject)
    }
}
