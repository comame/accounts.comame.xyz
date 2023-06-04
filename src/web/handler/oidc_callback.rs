use http::enc::form_urlencoded;
use http::request::Request;
use http::response::Response;
use url::Url;

use crate::data::oidc_flow::authentication_flow_state::OidcFlow;
use crate::data::oidc_flow::authentication_response::AuthenticationResponse;
use crate::data::openid_provider::OpenIDProvider;
use crate::oidc::relying_party::callback;
use crate::web::static_file;

fn response_bad_request(msg: &str) -> Response {
    let mut res = Response::new();
    res.status = 403;
    res.body = Some(format!(r#"{{error: "{}"}}"#, msg));
    res
}

fn response_redirect(location: &str) -> Response {
    let mut res = Response::new();
    res.status = 302;
    res.headers.insert("Location".into(), location.into());
    res
}

fn response_permission_denied(client_id: &str) -> Response {
    let mut res = Response::new();
    let f = static_file::read("/error.html").unwrap();
    let f = f.replace("$RP", client_id);
    res.body = Some(f);
    res
}

pub async fn handler(req: &Request, op: OpenIDProvider, remote_addr: &str) -> Response {
    let query = req.query.clone();
    if query.is_none() {
        return response_bad_request("bad_request");
    }
    let query = query.unwrap();
    let query = form_urlencoded::parse(&query);
    if query.is_err() {
        return response_bad_request("melformed_query_format");
    }
    let query = query.unwrap();

    let state = query.get("state").cloned();
    let code = query.get("code").cloned();

    if state.is_none() || code.is_none() {
        return response_bad_request("missing_parameters");
    }

    let state = state.unwrap();
    let code = code.unwrap();

    let cookie = req.cookies.get("rp").cloned();
    if cookie.is_none() {
        return response_bad_request("no_session");
    }
    let state_id = cookie.unwrap();

    let result = callback(&state_id, &state, &code, op, remote_addr).await;

    if let Err(err) = result {
        if err.redirect_uri.is_none() {
            dbg!("invalid");
            return response_permission_denied(&err.client_id);
        }
        match err.flow.unwrap() {
            OidcFlow::Code => {
                let redirect_uri = err.redirect_uri.unwrap();
                let error_body = err.response;
                let mut redirect_uri = Url::parse(&redirect_uri).unwrap();
                redirect_uri
                    .query_pairs_mut()
                    .append_pair("error", error_body.error.to_string().as_str());
                if let Some(state) = error_body.state {
                    redirect_uri.query_pairs_mut().append_pair("state", &state);
                }
                return response_redirect(redirect_uri.as_str());
            }
            OidcFlow::Implicit => {
                let redirect_uri = err.redirect_uri.unwrap();
                let error_body = err.response;
                let mut hash = String::new();
                hash.push_str(&format!(
                    "error={}",
                    http::enc::url_encode::encode(error_body.error.to_string().as_str())
                ));
                if let Some(state) = error_body.state {
                    hash.push_str(&format!("&state={}", http::enc::url_encode::encode(&state)))
                }
                return response_redirect(&format!("{redirect_uri}#{hash}"));
            }
        }
    }

    let result = result.unwrap();

    match result.response {
        AuthenticationResponse::Code(res) => {
            let mut redirect_uri = Url::parse(result.redirect_uri.as_str()).unwrap();

            redirect_uri
                .query_pairs_mut()
                .append_pair("code", &res.code);
            if let Some(ref state) = res.state {
                redirect_uri.query_pairs_mut().append_pair("state", state);
            }
            response_redirect(redirect_uri.as_str())
        }
        AuthenticationResponse::Implicit(res) => {
            let mut hash = String::new();

            hash.push_str(&format!(
                "id_token={}",
                http::enc::url_encode::encode(&res.id_token)
            ));
            if let Some(ref state) = res.state {
                hash.push_str(&format!("&state={}", http::enc::url_encode::encode(state)));
            }

            let redirect_uri = format!("{}#{}", result.redirect_uri, hash);
            response_redirect(redirect_uri.as_str())
        }
    }
}
