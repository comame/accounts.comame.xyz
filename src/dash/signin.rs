use std::env;

use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::{from_str, to_string};

use crate::crypto::rand::random_str;
use crate::data::oidc_flow::code_request::CodeRequest;
use crate::data::oidc_flow::code_response::CodeResponse;
use crate::db::redis;
use crate::enc::url::encode;
use crate::http::data::tools_id_token::{IdTokenRequest, IdTokenResponse};
use crate::http::parse_body::parse_body;
use crate::time::now;

const PREFIX: &str = "DASH-SIGN";

/// Returns redirect url
pub fn signin() -> String {
    let state = random_str(16);
    let nonce = random_str(16);
    let origin = env::var("HOST").unwrap();

    let redis_key = format!("{PREFIX}:{state}:{nonce}");
    redis::set(&redis_key, "_", 60);

    let redirect_uri = format!("{origin}/dash/callback");
    format!("{origin}/authenticate?client_id=accounts.comame.xyz&redirect_uri={redirect_uri}&scope=openid+code&response_type=code&state={state}&nonce={nonce}&prompt=login")
}

/// Returns token
pub async fn callback(state: &str, code: &str) -> Result<String, ()> {
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let origin = env::var("HOST").unwrap();
    let redirect_uri = format!("{origin}/dash/callback");

    let client_id = "accounts.comame.xyz";
    let client_secret = &env::var("CLIENT_SECRET").unwrap();

    let code_request = CodeRequest {
        grant_type: "authorization_code".to_string(),
        code: code.to_string(),
        redirect_uri,
        client_id: client_id.to_string(),
        client_secret: Some(client_secret.to_string()),
    };

    let code_request_str = format!(
        "grant_type={}&code={}&redirect_uri={}&client_id={}&client_secret={}",
        code_request.grant_type,
        code_request.code,
        encode(&code_request.redirect_uri),
        code_request.client_id,
        code_request.client_secret.unwrap(),
    );

    let code_request = Request::builder()
        .method(Method::POST)
        .uri(format!("{origin}/code"))
        .body(Body::from(code_request_str))
        .unwrap();

    let code_response = client.request(code_request).await;
    if code_response.is_err() {
        dbg!("invalid");
        return Err(());
    }

    let code_response = code_response.unwrap();
    let code_response = parse_body(code_response.into_body()).await.unwrap();
    let code_response = from_str::<CodeResponse>(&code_response);
    if code_response.is_err() {
        dbg!("invalid");
        return Err(());
    }

    let id_token = code_response.unwrap().id_token;

    let session_request = IdTokenRequest {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        id_token,
    };
    let session_request = Request::builder()
        .method(Method::POST)
        .uri(format!("{origin}/tools/id-token"))
        .body(Body::from(to_string(&session_request).unwrap()))
        .unwrap();
    let session_response = client.request(session_request).await;
    if session_response.is_err() {
        dbg!("invalid");
        return Err(());
    }
    let session_response = session_response.unwrap();
    let session_response = parse_body(session_response.into_body()).await;
    if session_response.is_err() {
        dbg!("invalid");
        return Err(());
    }
    let session_response = session_response.unwrap();
    let session_response = from_str::<IdTokenResponse>(&session_response);
    if session_response.is_err() {
        dbg!("invalid");
        return Err(());
    }
    let session_response = session_response.unwrap();

    let claim = session_response.claim;
    let token = session_response.session;

    if claim.nonce.is_none() {
        dbg!("invalid");
        return Err(());
    }

    let redis_key = format!("{PREFIX}:{state}:{}", claim.nonce.unwrap());
    let redis_value = redis::get(&redis_key);
    if redis_value.is_none() {
        dbg!("invalid");
        return Err(());
    }
    redis::del(&redis_key);

    if claim.aud != "accounts.comame.xyz" {
        dbg!("invalid");
        return Err(());
    }

    if claim.sub != "admin" {
        dbg!("invalid");
        return Err(());
    }

    if now() - claim.auth_time > 60 {
        dbg!("invalid");
        return Err(());
    }

    Ok(token)
}
