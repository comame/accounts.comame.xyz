use http::{
    query_builder::QueryBuilder,
    request::{Method, Request},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::from_str;

use crate::{
    crypto::rand,
    data::{
        jwk::{self, Jwk},
        oidc_flow::{
            code_request::CodeRequest, code_response::CodeResponse, id_token_claim::IdTokenClaim,
        },
        openid_provider::OpenIDProvider,
    },
    web::fetch::{self, fetch},
};

fn redirect_uri() -> String {
    format!("{}/oidc-callback/google", std::env::var("HOST").unwrap())
}

/// リダイレクト先の URL を返す
pub fn generate_authentication_endpoint_url() -> String {
    // 現時点では Google にのみ対応しているので、適当にハードコードしておく

    let client_id = std::env::var("GOOGLE_OIDC_CLIENT_ID").unwrap();
    let redirect_uri = redirect_uri();
    let state = rand::random_str(16);
    let nonce = rand::random_str(16);

    // TODO: ちゃんと状態を保存するようにする

    let endpoint = "https://accounts.google.com/o/oauth2/v2/auth";

    let query = QueryBuilder::new()
        .append("client_id", &client_id)
        .append("response_type", "code")
        .append("scope", "openid email profile")
        .append("redirect_uri", &redirect_uri)
        .append("state", &state)
        .append("nonce", &nonce)
        .build();

    format!("{endpoint}?{query}")
}

pub async fn callback(state: &str, code: &str, op: OpenIDProvider) -> Result<IdTokenClaim, ()> {
    let client_id = match op {
        OpenIDProvider::Google => std::env::var("GOOGLE_OIDC_CLIENT_ID").unwrap(),
    };
    let client_secret = match op {
        OpenIDProvider::Google => std::env::var("GOOGLE_OIDC_CLIENT_SECRET").unwrap(),
    };

    // TODO: state, nonce を検証する

    let body = CodeRequest {
        grant_type: "authorization_code".into(),
        code: code.into(),
        redirect_uri: redirect_uri(),
        client_id,
        client_secret: Some(client_secret),
    };
    // この辺は適当にハードコードしておく
    let mut token_request = Request::new("/token", Some(&body.to_string()));
    token_request.origin = Some("https://oauth2.googleapis.com/".into());
    token_request.method = Method::Post;
    token_request.headers.insert(
        "Content-Type".into(),
        "application/x-www-form-urlencoded".into(),
    );

    let res = fetch(&token_request).await;
    if res.is_err() {
        return Err(());
    }
    let res = res.unwrap();

    if res.status != 200 {
        return Err(());
    }

    let body = from_str::<CodeResponse>(&res.body.unwrap());
    if let Err(err) = body {
        dbg!(&err);
        return Err(());
    }
    let body = body.unwrap();

    let id_token = body.id_token;

    let id_token_header = jsonwebtoken::decode_header(&id_token);
    if let Err(err) = id_token_header {
        dbg!(&err);
        return Err(());
    }
    let id_token_header = id_token_header.unwrap();

    if id_token_header.alg != Algorithm::RS256 {
        dbg!("unsupported JWT algorithm");
        return Err(());
    }

    if id_token_header.kid.is_none() {
        dbg!("kid is none");
        return Err(());
    }
    let kid = id_token_header.kid.unwrap();

    // とりあえずハードコード
    let mut jwk_request = Request::new("/oauth2/v3/certs", None);
    jwk_request.origin = Some("https://www.googleapis.com".into());
    let jwk_response = fetch(&jwk_request).await;

    if let Err(_) = jwk_response {
        return Err(());
    }
    let jwk_response = jwk_response.unwrap();

    let jwk = from_str::<Jwk>(&jwk_response.body.unwrap());
    if let Err(err) = jwk {
        dbg!(&err);
        return Err(());
    }
    let jwk = jwk.unwrap();

    let jwk = jwk.keys.iter().find(|v| v.kid == kid).cloned();
    if jwk.is_none() {
        dbg!("target kid not found");
        return Err(());
    }
    let jwk = jwk.unwrap();

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);
    if let Err(e) = decoding_key {
        dbg!(&e);
        return Err(());
    }
    let decoding_key = decoding_key.unwrap();

    let claim = jsonwebtoken::decode::<IdTokenClaim>(
        &id_token,
        &decoding_key,
        &Validation::new(Algorithm::RS256),
    );
    if let Err(err) = claim {
        dbg!(&err);
        return Err(());
    }

    let claim = claim.unwrap().claims;
    dbg!(&claim);

    Ok(claim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let url = generate_authentication_endpoint_url();
        assert!(url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
    }
}
