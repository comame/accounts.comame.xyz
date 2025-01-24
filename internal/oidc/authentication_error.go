package oidc

type AuthenticationError string

var (
	ErrAuthenticationErrInvalidRequest           AuthenticationError = "invalid_request"
	ErrAuthenticationErrUnauthorizedClient       AuthenticationError = "unauthorized_client"
	ErrAuthenticationErrAccessDenied             AuthenticationError = "access_denied"
	ErrAuthenticationErrUnsupportedResponseType  AuthenticationError = "unsupported_response_type"
	ErrAuthenticationErrInvalidScope             AuthenticationError = "invalid_scope"
	ErrAuthenticationErrServerError              AuthenticationError = "server_error"
	ErrAuthenticationErrTemporarilyUnavailable   AuthenticationError = "temporarily_unavailable"
	ErrAuthenticationErrInteractionRequired      AuthenticationError = "interaction_required"
	ErrAuthenticationErrLoginRequired            AuthenticationError = "login_required"
	ErrAuthenticationErrAccountSelectionRequired AuthenticationError = "account_selection_required"
	ErrAuthenticationErrConsentRequired          AuthenticationError = "consent_required"
	ErrAuthenticationErrInvalidRequestUri        AuthenticationError = "invalid_request_uri"
	ErrAuthenticationErrInvalidRequestObject     AuthenticationError = "invalid_request_object"
	ErrAuthenticationErrRequestNotSupported      AuthenticationError = "request_not_supported"
	ErrAuthenticationErrRequestUriNotSupported   AuthenticationError = "request_uri_not_supported"
	ErrAuthenticationErrRegistrationNotSupported AuthenticationError = "registration_not_supported"
)
