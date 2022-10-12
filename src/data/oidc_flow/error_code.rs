use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorCode {
    // Defined in OAuth 2.0 (RFC 6749)
    InvalidRequest,
    UnauthorizedClient,
    AccessDenied,
    UnsupportedResponseType,
    InvalidScope,
    ServerError,
    TemporarilyUnavailable,

    // Defined in OpenID Connect Core 1.0
    InteractionRequired,
    LoginRequired,
    AccountSelectionRequired,
    ConsentRequired,
    InvalidRequestUri,
    InvalidRequestObject,
    RequestNotSupported,
    RequestUriNotSupported,
    RegistrationNotSupported,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidRequest => "invalid_request",
                Self::UnauthorizedClient => "unauthorized_client",
                Self::AccessDenied => "access_denied",
                Self::UnsupportedResponseType => "unsupported_response_type",
                Self::InvalidScope => "invalid_scope",
                Self::ServerError => "server_error",
                Self::TemporarilyUnavailable => "temporarily_unavailable",
                Self::InteractionRequired => "interaction_required",
                Self::LoginRequired => "login_required",
                Self::AccountSelectionRequired => "account_selection_required",
                Self::ConsentRequired => "consent_required",
                Self::InvalidRequestUri => "invalid_request_uri",
                Self::InvalidRequestObject => "invalid_request_object",
                Self::RequestNotSupported => "request_not_supported",
                Self::RequestUriNotSupported => "request_uri_not_supported",
                Self::RegistrationNotSupported => "registration_not_supported",
            }
        )
    }
}
