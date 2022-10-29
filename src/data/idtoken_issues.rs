use crate::db::idtoken_issues::insert;

use super::oidc_flow::id_token_claim::IdTokenClaim;

pub struct IdTokenIssues {
    pub sub: String,
    pub aud: String,
    pub iat: u64,
}

impl IdTokenIssues {
    pub fn log(claim: &IdTokenClaim) {
        insert(&Self {
            sub: claim.sub.clone(),
            aud: claim.aud.clone(),
            iat: claim.iat,
        });
    }
}
