use hyper::{Body, Method, Request, Response, StatusCode};
use url::Url;

use crate::data::oidc_flow::authentication_flow_state::LoginRequirement;
use crate::data::oidc_flow::authentication_request::AuthenticationRequest;
use crate::enc::url as percent_encoding;
use crate::http::parse_body::parse_body;
use crate::http::set_header::set_header;
use crate::oidc::authentication_request::pre_authenticate;

fn response_bad_request() -> Response<Body> {
    let mut response = Response::new(Body::from(r#"{"message": "Bad Request"}"#));
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response
}

fn redirect(url: &str) -> Response<Body> {
    let mut response = Response::new(Body::empty());
    *response.status_mut() = StatusCode::FOUND;
    set_header(&mut response, "Location", url);
    response
}

pub async fn handler(req: Request<Body>) -> Response<Body> {
    let method = req.method();

    let mut authentication_request: Result<AuthenticationRequest, ()> = Err(());

    if method == Method::GET {
        let url = Url::parse(&format!("http://example.com{}", &req.uri().to_string())).unwrap();
        let query = url.query();
        if query.is_none() {
            return response_bad_request();
        }

        authentication_request = AuthenticationRequest::parse_query(query.unwrap());
    } else if method == Method::POST {
        let body = parse_body(req.into_body()).await;
        if body.is_err() {
            return response_bad_request();
        }

        authentication_request = AuthenticationRequest::parse_query(&body.unwrap());
    }

    dbg!(&authentication_request);
    if authentication_request.is_err() {
        return response_bad_request();
    }

    let result = pre_authenticate(authentication_request.unwrap());

    if let Err(err) = result {
        dbg!(&err);

        if err.redirect_uri.is_none() {
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