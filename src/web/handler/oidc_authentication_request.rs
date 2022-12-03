use std::borrow::BorrowMut;

use http::request::Method;
use http::{request::Request, response::Response};
use url::Url;

use crate::data::oidc_flow::authentication_flow_state::LoginRequirement;
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::enc::url as percent_encoding;
use crate::oidc::authentication_request::pre_authenticate;

fn response_bad_request() -> Response {
    let mut response = Response::new();
    response.status = 403;
    response.body = Some(r#"{"message": "Bad Request"}"#.to_string());
    response
}

fn redirect(url: &str) -> Response {
    let mut response = Response::new();
    response.status = 302;
    response
        .headers
        .borrow_mut()
        .insert("Location".to_string(), url.to_string());
    response
}

pub fn handler(req: Request) -> Response {
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

    let result = pre_authenticate(authentication_request.unwrap());

    if let Err(err) = result {
        if err.redirect_uri.is_none() {
            dbg!("invalid");
            return response_bad_request();
        }

        let redirect_url = &err.redirect_uri.unwrap();
        let mut uri = Url::parse(redirect_url).unwrap();

        let error_body = err.response;
        uri.query_pairs_mut()
            .append_pair("error", &error_body.error.to_string());
        if let Some(state) = error_body.state {
            uri.query_pairs_mut().append_pair("state", &state);
        }

        return redirect(uri.as_str());
    }

    // 正常系
    let state = result.unwrap();
    let sid = &state.id();
    let cid = percent_encoding::encode(&state.relying_party_id);
    let uri = match state.login_requirement {
        LoginRequirement::Consent => format!("/confirm?sid={sid}&cid={}", cid),
        LoginRequirement::ReAuthenticate => format!("/reauthenticate?sid={sid}&cid={}", cid),
        LoginRequirement::MaxAge => {
            format!(
                "/signin?sid={sid}&age={}&cid={}#maxage",
                state.max_age.unwrap(),
                cid
            )
        }
        LoginRequirement::None => format!("/signin?sid={sid}&cid={}#nointeraction", cid),
        LoginRequirement::Any => format!("/signin?sid={sid}&cid={}", cid),
    };
    redirect(&uri)
}
