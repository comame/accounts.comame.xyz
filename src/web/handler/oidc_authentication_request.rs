use std::borrow::BorrowMut;

use http::query_builder::QueryBuilder;
use http::request::{Method, Request};
use http::response::Response;

use crate::auth::session;
use crate::data::oidc_flow::authentication_flow_state::LoginRequirement;
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::oidc::authentication_request::pre_authenticate;
use crate::web::set_header::no_store;

fn response_bad_request() -> Response {
    let mut response = Response::new();
    response.status = 403;
    response.body = Some(r#"{"message": "Bad Request"}"#.to_string());
    no_store(&mut response);
    response
}

fn response_redirect(url: &str) -> Response {
    let mut response = Response::new();
    response.status = 302;
    response
        .headers
        .borrow_mut()
        .insert("Location".to_string(), url.to_string());
    no_store(&mut response);
    response
}

pub fn handler(req: &Request) -> Response {
    let req = req.clone();
    let method = req.method;

    let mut authentication_request: Result<AuthenticationRequest, ()> = Err(());

    if method == Method::Get {
        let query = req.query;
        if query.is_none() {
            dbg!("invalid");
            return response_bad_request();
        }

        authentication_request = AuthenticationRequest::parse_query(&query.unwrap());
    } else if method == Method::Post {
        if req.body.is_none() {
            dbg!("invalid");
            return response_bad_request();
        }

        authentication_request = AuthenticationRequest::parse_query(&req.body.unwrap());
    }

    if authentication_request.is_err() {
        dbg!("invalid");
        return response_bad_request();
    }

    let body = authentication_request.unwrap();
    let client_id = body.client_id.clone();

    let result = pre_authenticate(body);

    if let Err(err) = result {
        if err.redirect_uri.is_none() {
            dbg!("invalid");
            return response_bad_request();
        }

        let redirect_url = &err.redirect_uri.unwrap();

        let error_body = err.response;

        let mut query = QueryBuilder::new();
        query.append("error", &error_body.error.to_string());
        if let Some(state) = error_body.state {
            query.append("state", &state);
        }

        return response_redirect(&format!("{redirect_url}?{}", query.build()));
    }

    // 正常系
    let state = result.unwrap();
    let sid = &state.id();
    let cid = http::enc::url_encode::encode(&state.relying_party_id);

    let signin_url = format!("/signin?sid={sid}&cid={cid}");

    let session_key = req.cookies.get("Session");
    if session_key.is_none() {
        return response_redirect(&signin_url);
    }
    let session_key = session_key.unwrap();

    let session = session::authenticate(&client_id, session_key);
    if session.is_none() {
        return response_redirect(&signin_url);
    }
    let session = session.unwrap();

    let confirm_url = format!("/confirm?sid={sid}&cid={cid}&u={}", session.id);
    let reauthenticate_url = format!("/reauthenticate?sid={sid}&cid={cid}&u={}", session.id);

    match state.login_requirement {
        LoginRequirement::Consent => {
            response_redirect(&confirm_url)
        }
        LoginRequirement::ReAuthenticate => {
            response_redirect(&reauthenticate_url)
        }
        LoginRequirement::MaxAge => {
            unimplemented!();
        }
        LoginRequirement::None => {
            unimplemented!("nointaeraction を実装");
        }
        LoginRequirement::Any => {
            response_redirect(&confirm_url)
        }
    }
}
