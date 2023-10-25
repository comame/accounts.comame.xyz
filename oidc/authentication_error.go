package oidc

import "errors"

type AuthenticationError error

var (
	ErrAuthenticationErrInvalidRequest           AuthenticationError = errors.New("invalid_request")
	ErrAuthenticationErrUnauthorizedClient       AuthenticationError = errors.New("unauthorized_client")
	ErrAuthenticationErrAccessDenied             AuthenticationError = errors.New("access_denied")
	ErrAuthenticationErrUnsupportedResponseType  AuthenticationError = errors.New("unsupported_response_type")
	ErrAuthenticationErrInvalidScope             AuthenticationError = errors.New("invalid_scope")
	ErrAuthenticationErrServerError              AuthenticationError = errors.New("server_error")
	ErrAuthenticationErrTemporarilyUnavailable   AuthenticationError = errors.New("temporarily_unavailable")
	ErrAuthenticationErrInteractionRequired      AuthenticationError = errors.New("interaction_required")
	ErrAuthenticationErrLoginRequired            AuthenticationError = errors.New("login_required")
	ErrAuthenticationErrAccountSelectionRequired AuthenticationError = errors.New("account_selection_required")
	ErrAuthenticationErrConsentRequired          AuthenticationError = errors.New("consent_required")
	ErrAuthenticationErrInvalidRequestUri        AuthenticationError = errors.New("invalid_request_uri")
	ErrAuthenticationErrInvalidRequestObject     AuthenticationError = errors.New("invalid_request_object")
	ErrAuthenticationErrRequestNotSupported      AuthenticationError = errors.New("request_not_supported")
	ErrAuthenticationErrRequestUriNotSupported   AuthenticationError = errors.New("request_uri_not_supported")
	ErrAuthenticationErrRegistrationNotSupported AuthenticationError = errors.New("registration_not_supported")
)
