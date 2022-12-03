use std::fmt::Display;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::enc::base64::encode_base64_url;
use crate::enc::url::encode;
use crate::web::parse_body::parse_body;
use crate::web::set_header::set_header_req;

static mut ACCESS_TOKEN: Option<String> = None;

#[derive(Serialize)]
struct SendGmailAPIRequest {
    pub raw: String,
}

struct MailBody {
    pub subject: String,
    pub to: String,
    pub from: String,
    pub body: String,
}

impl Display for MailBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = encode_base64_url(
            format!(
                "Subject: {}\r\nTo: {}\r\nFrom: {}\r\n\r\n{}",
                self.subject, self.to, self.from, self.body
            )
            .as_bytes(),
        );
        write!(f, "{text}")
    }
}

fn get_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build::<_, hyper::Body>(HttpsConnector::new())
}

pub fn send_mail(subject: &str, to: &str, body: &str) -> Result<(), ()> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_send_mail(subject, to, body))
}

pub async fn async_send_mail(subject: &str, to: &str, body: &str) -> Result<(), ()> {
    let body = MailBody {
        subject: subject.to_string(),
        to: to.to_string(),
        from: std::env::var("MAIL_FROM").unwrap(),
        body: body.to_string(),
    };

    let token = get_current_token();
    if let Some(token) = token {
        let result = internal_send_mail(&body, &token).await;
        if result.is_err() {
            get_token().await?;
            let token = get_current_token();
            if token.is_none() {
                return Err(());
            }
            internal_send_mail(&body, &token.unwrap()).await?;
            return Ok(());
        }
        return Ok(());
    }

    get_token().await?;
    let token = get_current_token();
    if token.is_none() {
        return Err(());
    }
    internal_send_mail(&body, &token.unwrap()).await?;
    Ok(())
}

async fn internal_send_mail(body: &MailBody, token: &str) -> Result<(), ()> {
    let endpoint = "https://gmail.googleapis.com/gmail/v1/users/me/messages/send";

    let client = get_client();

    let request_body = SendGmailAPIRequest {
        raw: body.to_string(),
    };

    let mut request = Request::builder()
        .method(Method::POST)
        .uri(endpoint)
        .body(Body::from(to_string(&request_body).unwrap()))
        .unwrap();

    set_header_req(&mut request, "Authorization", &format!("Bearer {token}"));
    set_header_req(&mut request, "Content-Type", "application/json");

    let response = client.request(request).await;

    if response.is_ok() {
        let response = response.unwrap();
        if response.status() == StatusCode::OK {
            dbg!("sent: {}", &body.to);
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

struct TokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: String,
}

impl Display for TokenRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "client_id={}&client_secret={}&refresh_token={}&grant_type=refresh_token",
            encode(&self.client_id),
            encode(&self.client_secret),
            encode(&self.refresh_token)
        )
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    pub access_token: String,
}

async fn get_token() -> Result<(), ()> {
    let endpoint = "https://oauth2.googleapis.com/token";
    let client = get_client();

    let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap();
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap();
    let refresh_token = std::env::var("GOOGLE_REFRESH_TOKEN").unwrap();

    let mut request = Request::builder()
        .uri(endpoint)
        .method(Method::POST)
        .body(Body::from(
            TokenRequest {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                refresh_token: refresh_token.to_string(),
            }
            .to_string(),
        ))
        .unwrap();

    set_header_req(
        &mut request,
        "Content-Type",
        "application/x-www-form-urlencoded",
    );

    let result = client.request(request).await;
    if let Err(err) = result {
        dbg!(err);
        return Err(());
    }
    let result = result.unwrap();

    let body = parse_body(result.into_body()).await?;

    let body = from_str::<TokenResponse>(&body);
    if body.is_err() {
        return Err(());
    }

    let token = body.unwrap().access_token;

    set_token(&token);

    dbg!("Google Token Refreshed");

    Ok(())
}

fn set_token(token: &str) {
    unsafe { ACCESS_TOKEN = Some(token.to_string()) }
}

fn get_current_token() -> Option<String> {
    unsafe { ACCESS_TOKEN.clone() }
}
