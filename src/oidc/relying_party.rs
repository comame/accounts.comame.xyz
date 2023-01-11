use http::query_builder::QueryBuilder;
use http::request::{Method, Request};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde_json::from_str;

use super::authentication_flow_state::get_state_keep;
use super::authentication_request::{
    post_authentication, AuthenticationError, PostAuthenticationResponse,
};
use crate::crypto::rand;
use crate::data::authentication::{Authentication, AuthenticationMethod};
use crate::data::authentication_failure::AuthenticationFailure;
use crate::data::jwk::Jwk;
use crate::data::oidc_flow::authenticationi_error_response::AuthenticationErrorResponse;
use crate::data::oidc_flow::code_request::CodeRequest;
use crate::data::oidc_flow::code_response::CodeResponse;
use crate::data::oidc_flow::error_code::ErrorCode;
use crate::data::oidc_flow::id_token_claim::IdTokenClaim;
use crate::data::oidc_flow::relying_party_state::RelyingPartyState;
use crate::data::oidc_flow::userinfo::UserInfo;
use crate::data::op_user::OpUser;
use crate::data::openid_provider::OpenIDProvider;
use crate::data::role::Role;
use crate::data::role_access::RoleAccess;
use crate::data::user::User;
use crate::data::user_role::UserRole;
use crate::time::now;
use crate::web::fetch::fetch;

fn redirect_uri() -> String {
    format!("{}/oidc-callback/google", std::env::var("HOST").unwrap())
}

pub struct AuthorizationInitate {
    pub redirect_url: String,
    pub state_id: String,
}

pub fn generate_authentication_endpoint_url(
    authorization_flow_state_id: &str,
    op: OpenIDProvider,
    user_agent_id: &str,
) -> AuthorizationInitate {
    // TODO: 他に必要な OpenID Connect のパラメータを引き渡す (prompt とか)

    // 現時点では Google にのみ対応しているので、適当にハードコードしておく
    if op != OpenIDProvider::Google {
        unimplemented!();
    }

    let client_id = std::env::var("GOOGLE_OIDC_CLIENT_ID").unwrap();
    let redirect_uri = redirect_uri();
    let state = rand::random_str(16);
    let nonce = rand::random_str(16);

    let saved_state = RelyingPartyState {
        state_id: authorization_flow_state_id.into(),
        nonce: nonce.clone(),
        state: state.clone(),
        op,
        user_agent_id: user_agent_id.into(),
    };
    RelyingPartyState::save(&saved_state);

    let endpoint = "https://accounts.google.com/o/oauth2/v2/auth";

    let query = QueryBuilder::new()
        .append("client_id", &client_id)
        .append("response_type", "code")
        .append("scope", "openid email profile")
        .append("redirect_uri", &redirect_uri)
        .append("state", &state)
        .append("nonce", &nonce)
        .build();

    let redirect_url = format!("{endpoint}?{query}");

    AuthorizationInitate {
        redirect_url,
        state_id: saved_state.state_id,
    }
}

pub async fn callback(
    state_id: &str,
    state: &str,
    code: &str,
    op: OpenIDProvider,
    remote_addr: &str,
) -> Result<PostAuthenticationResponse, AuthenticationError> {
    let client_id = match op {
        OpenIDProvider::Google => std::env::var("GOOGLE_OIDC_CLIENT_ID").unwrap(),
    };
    let client_secret = match op {
        OpenIDProvider::Google => std::env::var("GOOGLE_OIDC_CLIENT_SECRET").unwrap(),
    };

    let saved_state = RelyingPartyState::get_consume(state_id);
    if saved_state.is_none() {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::InvalidRequest,
                state: None,
            },
        });
    }
    let saved_relying_party_state = saved_state.unwrap();

    let user_agent_id = saved_relying_party_state.user_agent_id;

    if saved_relying_party_state.state != state {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::InvalidRequest,
                state: None,
            },
        });
    }

    let state = saved_relying_party_state.state;

    if saved_relying_party_state.op != op {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::InvalidRequest,
                state: Some(state),
            },
        });
    }

    let body = CodeRequest {
        grant_type: "authorization_code".into(),
        code: code.into(),
        redirect_uri: redirect_uri(),
        client_id: client_id.clone(),
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
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let res = res.unwrap();

    if res.status != 200 {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    let body = from_str::<CodeResponse>(&res.body.unwrap());
    if let Err(err) = body {
        dbg!(&err);
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let body = body.unwrap();

    let access_token = body.access_token;
    let id_token = body.id_token;

    let id_token_header = jsonwebtoken::decode_header(&id_token);
    if let Err(err) = id_token_header {
        dbg!(&err);
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let id_token_header = id_token_header.unwrap();

    if id_token_header.alg != Algorithm::RS256 {
        dbg!("unsupported JWT algorithm");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if id_token_header.kid.is_none() {
        dbg!("kid is none");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let kid = id_token_header.kid.unwrap();

    // とりあえずハードコード
    let mut jwk_request = Request::new("/oauth2/v3/certs", None);
    jwk_request.origin = Some("https://www.googleapis.com".into());
    let jwk_response = fetch(&jwk_request).await;

    if jwk_response.is_err() {
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let jwk_response = jwk_response.unwrap();

    let jwk = from_str::<Jwk>(&jwk_response.body.unwrap());
    if let Err(err) = jwk {
        dbg!(&err);
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let jwk = jwk.unwrap();

    let jwk = jwk.keys.iter().find(|v| v.kid == kid).cloned();
    if jwk.is_none() {
        dbg!("target kid not found");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let jwk = jwk.unwrap();

    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);
    if let Err(e) = decoding_key {
        dbg!(&e);
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let decoding_key = decoding_key.unwrap();

    let claim = jsonwebtoken::decode::<IdTokenClaim>(
        &id_token,
        &decoding_key,
        &Validation::new(Algorithm::RS256),
    );
    if let Err(err) = claim {
        dbg!(&err);
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    let claim = claim.unwrap().claims;

    if claim.iss != "https://accounts.google.com" {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if now() > claim.exp {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if now() < claim.iat {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if claim.aud != client_id {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if claim.nonce.is_none() {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }

    if *claim.nonce.as_ref().unwrap() != saved_relying_party_state.nonce {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::InvalidRequest,
                state: None,
            },
        });
    }

    let user_id = claim.sub.clone();
    let login_type = match op {
        OpenIDProvider::Google => AuthenticationMethod::Google,
    };

    let user_id = match op {
        OpenIDProvider::Google => format!("google:{user_id}"),
    };

    let saved_state = get_state_keep(state_id);
    if saved_state.is_none() {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let saved_authentication_flow_state = saved_state.unwrap();

    let relying_party_id = saved_authentication_flow_state.relying_party_id;

    let mut userinfo_request = Request::new("/v1/userinfo".into(), None);
    userinfo_request.origin = Some("https://openidconnect.googleapis.com".into());
    userinfo_request
        .headers
        .insert("Authorization".into(), format!("Bearer {access_token}"));
    let userinfo_response = fetch(&userinfo_request).await;

    if let Err(_) = userinfo_response {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let userinfo_response = userinfo_response.unwrap();
    let userinfo_response = from_str::<UserInfo>(&userinfo_response.body.unwrap());
    if let Err(_) = userinfo_response {
        dbg!("invalid");
        return Err(AuthenticationError {
            redirect_uri: None,
            flow: None,
            response: AuthenticationErrorResponse {
                error: ErrorCode::ServerError,
                state: None,
            },
        });
    }
    let mut userinfo_response = userinfo_response.unwrap();

    let user_exists = User::find(&user_id).is_some();
    if !user_exists {
        let result = User::new(&user_id);
        if let Err(_) = result {
            dbg!("invalid");
            return Err(AuthenticationError {
                redirect_uri: None,
                flow: None,
                response: AuthenticationErrorResponse {
                    error: ErrorCode::ServerError,
                    state: None,
                },
            });
        }

        let role_exists = Role::get(&op.to_string()).is_some();
        if !role_exists {
            Role::new(&op.to_string());
        }

        UserRole::new(&user_id, &op.to_string()).unwrap();
    }

    userinfo_response.sub = user_id.clone(); // OP ごとの prefix 付きのものに差し替える
    UserInfo::insert(&userinfo_response);

    let op_user = OpUser::get(&claim.sub, OpenIDProvider::Google);
    if let Some(op_user) = op_user {
        // 既存のユーザーに対して紐づけがある場合
        userinfo_response.sub = op_user.user_id.clone();
        UserInfo::insert(&userinfo_response);

        let is_accessable_user = RoleAccess::is_accessible(&op_user.user_id, &relying_party_id);
        if !is_accessable_user {
            AuthenticationFailure::new(
                &op_user.user_id,
                &crate::data::authentication::AuthenticationMethod::Session,
                &crate::data::authentication_failure::AuthenticationFailureReason::NoUserBinding,
                remote_addr,
            );
            dbg!("invalid");
            return Err(AuthenticationError {
                redirect_uri: None,
                flow: None,
                response: AuthenticationErrorResponse {
                    error: ErrorCode::UnauthorizedClient,
                    state: None,
                },
            });
        }
        Authentication::create(
            now(),
            &relying_party_id,
            &op_user.user_id,
            AuthenticationMethod::Google,
            &user_agent_id,
        );
        post_authentication(
            &op_user.user_id,
            state_id,
            &relying_party_id,
            &user_agent_id,
            login_type,
            remote_addr,
        )
    } else {
        // 紐づけがない場合
        if !RoleAccess::is_accessible(&user_id, &relying_party_id) {
            dbg!("invalid");
            return Err(AuthenticationError {
                redirect_uri: None,
                flow: None,
                response: AuthenticationErrorResponse {
                    error: ErrorCode::UnauthorizedClient,
                    state: None,
                },
            });
        }

        Authentication::create(
            now(),
            &relying_party_id,
            &user_id,
            AuthenticationMethod::Google,
            &user_agent_id,
        );

        let result = post_authentication(
            &user_id,
            state_id,
            &relying_party_id,
            &user_agent_id,
            login_type,
            remote_addr,
        );

        // TODO: 成功時にセッションを発行する
        // ただし、ユーザーの存在確認をする必要はないかもしれない (外部アカウントなので)

        result
    }
}
