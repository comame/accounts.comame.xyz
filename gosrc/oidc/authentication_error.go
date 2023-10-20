package oidc

import "errors"

type AuthenticationError error

var (
	AuthenticationErrInvalidRequest           AuthenticationError = errors.New("invalid_request")
	AuthenticationErrUnauthorizedClient       AuthenticationError = errors.New("unauthorized_client")
	AuthenticationErrAccessDenied             AuthenticationError = errors.New("access_denied")
	AuthenticationErrUnsupportedResponseType  AuthenticationError = errors.New("unsupported_response_type")
	AuthenticationErrInvalidScope             AuthenticationError = errors.New("invalid_scope")
	AuthenticationErrServerError              AuthenticationError = errors.New("server_error")
	AuthenticationErrTemporarilyUnavailable   AuthenticationError = errors.New("temporarily_unavailable")
	AuthenticationErrInteractionRequired      AuthenticationError = errors.New("interaction_required")
	AuthenticationErrLoginRequired            AuthenticationError = errors.New("login_required")
	AuthenticationErrAccountSelectionRequired AuthenticationError = errors.New("account_selection_required")
	AuthenticationErrConsentRequired          AuthenticationError = errors.New("consent_required")
	AuthenticationErrInvalidRequestUri        AuthenticationError = errors.New("invalid_request_uri")
	AuthenticationErrInvalidRequestObject     AuthenticationError = errors.New("invalid_request_object")
	AuthenticationErrRequestNotSupported      AuthenticationError = errors.New("request_not_supported")
	AuthenticationErrRequestUriNotSupported   AuthenticationError = errors.New("request_uri_not_supported")
	AuthenticationErrRegistrationNotSupported AuthenticationError = errors.New("registration_not_supported")
)
