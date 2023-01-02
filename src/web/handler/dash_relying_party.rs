use http::request::Request;
use http::response::Response;
use serde_json::{from_str, to_string};

use crate::dash::relying_party;
use crate::dash::signin::validate_token;
use crate::data::openid_provider::OpenIDProvider;
use crate::web::data::dash_rp_request::{
    RelyingPartyAddRedirectUriRequest, RelyingPartyBindingRequest, RelyingPartyClientIdRequest,
    RelyingPartyFederatedUserBindingRequest,
};
use crate::web::data::dash_rp_response::{
    RelyingPartiesResponse, RelyingPartyBindingResponse, RelyingPartyFederatedUserBindingResponse,
    RelyingPartyRawSecretResponse,
};
use crate::web::data::dash_standard_request::StandardRequest;

#[inline]
fn response_unauthorized() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"message": "unauthorized"}"#.to_string());
    res.status = 403;
    res
}

#[inline]
fn response_bad_request() -> Response {
    let mut res = Response::new();
    res.body = Some(r#"{"message": "Bad Request"}"#.to_string());
    res.status = 403;
    res
}

pub fn list_rp(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<StandardRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::list();

    let response = RelyingPartiesResponse { values: result };

    let mut res = Response::new();
    res.body = Some(to_string(&response).unwrap());
    res
}

pub fn create_rp(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyClientIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::create(&body.client_id);
    if result.is_err() {
        return response_bad_request();
    }
    let result = result.unwrap();

    let response = RelyingPartyRawSecretResponse {
        client_id: result.rp.client_id,
        client_secret: result.raw_secret,
    };

    let mut res = Response::new();
    res.body = Some(to_string(&response).unwrap());
    res
}

pub fn update_secret(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyClientIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::update_secret(&body.client_id);
    if result.is_err() {
        return response_bad_request();
    }

    let result = result.unwrap();

    let response = RelyingPartyRawSecretResponse {
        client_id: result.rp.client_id,
        client_secret: result.raw_secret,
    };

    let mut res = Response::new();
    res.body = Some(to_string(&response).unwrap());
    res
}

pub fn delete_rp(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyClientIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::delete(&body.client_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn add_redirect_uri(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyAddRedirectUriRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let _result = relying_party::add_redirect_uri(&body.client_id, &body.redirect_uri);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn delete_redirect_uri(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyAddRedirectUriRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::remove_redirect_uri(&body.client_id, &body.redirect_uri);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn list_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyClientIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::list_user_binding(&body.client_id);

    let mut res = Response::new();
    res.body = Some(
        to_string(&RelyingPartyBindingResponse {
            values: result.unwrap(),
        })
        .unwrap(),
    );
    res
}

pub fn add_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyBindingRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::add_user_binding(&body.client_id, &body.user_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn remove_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyBindingRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    relying_party::remove_user_binding(&body.client_id, &body.user_id);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn list_federated_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyClientIdRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let result = relying_party::list_federated_user_binding(&body.client_id);

    let mut res = Response::new();
    res.body =
        Some(to_string(&RelyingPartyFederatedUserBindingResponse { values: result }).unwrap());
    res
}

pub fn add_federated_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyFederatedUserBindingRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let issuer = OpenIDProvider::parse(&body.issuer);
    if let Err(_) = issuer {
        return response_bad_request();
    }
    let issuer = issuer.unwrap();

    relying_party::add_federated_user_binding(&body.client_id, issuer);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}

pub fn remove_federated_user_binding(req: &Request) -> Response {
    let req = req.clone();
    let body = from_str::<RelyingPartyFederatedUserBindingRequest>(&req.body.unwrap());
    if body.is_err() {
        return response_bad_request();
    }
    let body = body.unwrap();

    if !validate_token(&body.token) {
        return response_unauthorized();
    }

    let issuer = OpenIDProvider::parse(&body.issuer);
    if let Err(_) = issuer {
        return response_bad_request();
    }
    let issuer = issuer.unwrap();

    relying_party::remove_federated_user_bindng(&body.client_id, issuer);

    let mut res = Response::new();
    res.body = Some("{}".to_string());
    res
}
