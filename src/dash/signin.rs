use std::env;

use hyper::body::to_bytes;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde_json::{from_str, to_string};

use crate::crypto::rand::random_str;
use crate::data::oidc_flow::code_request::CodeRequest;
use crate::data::oidc_flow::code_response::CodeResponse;
use crate::db::redis;
use crate::time::now;
use crate::web::data::tools_id_token::{IdTokenRequest, IdTokenResponse};

const PREFIX_NONCE: &str = "DASH-SIGN";
const PREFIX_TOKEN: &str = "DASH-TOKEN";

/// Returns redirect url
pub fn signin() -> String {
    let state = random_str(16);
    let nonce = random_str(16);
    let origin = env::var("HOST").unwrap();

    let redis_key = format!("{PREFIX_NONCE}:{state}:{nonce}");
    redis::set(&redis_key, "_", 60);

    let redirect_uri = format!("{origin}/dash/callback");
    format!("{origin}/authenticate?client_id=accounts.comame.xyz&redirect_uri={redirect_uri}&scope=openid&response_type=code&state={state}&nonce={nonce}&prompt=login")
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
        http::enc::url_encode::encode(&code_request.redirect_uri),
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

    if claim.nonce.is_none() {
        dbg!("invalid");
        return Err(());
    }

    let nonce_redis_key = format!("{PREFIX_NONCE}:{state}:{}", claim.nonce.unwrap());
    let saved_nonce = redis::get(&nonce_redis_key);
    if saved_nonce.is_none() {
        dbg!("invalid");
        return Err(());
    }
    redis::del(&nonce_redis_key);

    if claim.aud != "accounts.comame.xyz" {
        dbg!("invalid");
        return Err(());
    }

    if claim.sub != "admin" {
        dbg!("invalid");
        return Err(());
    }

    if now() - claim.auth_time.unwrap() > 60 {
        dbg!("invalid");
        return Err(());
    }

    let token = random_str(60);
    let token_redis_key = format!("{PREFIX_TOKEN}:{token}");
    redis::set(&token_redis_key, "_", 5 * 60);

    Ok(token)
}

pub fn validate_token(token: &str) -> bool {
    let token_redis_key = format!("{PREFIX_TOKEN}:{token}");
    let token = redis::get(&token_redis_key);
    token.is_some()
}

#[deprecated]
async fn parse_body(body: Body) -> Result<String, ()> {
    let bytes = to_bytes(body).await;
    if let Err(err) = bytes {
        eprintln!("{}", err);
        return Err(());
    }

    let vec = bytes.unwrap().iter().cloned().collect::<Vec<u8>>();

    let str = String::from_utf8(vec);
    if let Err(err) = str {
        eprintln!("{}", err);
        return Err(());
    }

    Ok(str.unwrap())
}
