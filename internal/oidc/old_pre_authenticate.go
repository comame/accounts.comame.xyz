package oidc

import (
	"errors"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"golang.org/x/exp/slices"
)

var (
	errInvalidRedirectURI       = errors.New("invalid redirect_uri")
	errPromptIsUnsupported      = errors.New("prompt is unsupported")
	errMaxAgeIsUnsupported      = errors.New("max_age is unsupported")
	errMissingRequiredParameter = errors.New("missing required parameter")
)

type PreAuthenticateError struct {
	NotifyToClient bool
	cause          error
}

func (err PreAuthenticateError) Error() string {
	return err.cause.Error()
}

func PreAuthenticate(req AuthenticationRequest) (string, error) {
	throw := func(err error, notify bool) *PreAuthenticateError {
		return &PreAuthenticateError{
			cause:          err,
			NotifyToClient: notify,
		}
	}

	if req.Request != "" {
		return "", throw(ErrAuthenticationErrRequestNotSupported, true)
	}

	if req.Scope == "" || req.ResponseType == "" || req.ClientId == "" || req.RedirectURI == "" {
		return "", throw(errMissingRequiredParameter, false)
	}

	_, err := db.RelyingParty_select(req.ClientId)
	if err != nil {
		return "", throw(err, false)
	}

	uris, err := db.RelyingParty_selectRedirectURIs(req.ClientId)
	if err != nil {
		return "", throw(err, false)
	}

	if !slices.Contains(uris, req.RedirectURI) {
		return "", throw(errInvalidRedirectURI, false)
	}

	// TODO: 元のコードが openid profile email となっている
	if !hasScope(req.Scope, "openid") {
		return "", throw(ErrAuthenticationErrInvalidScope, true)
	}

	var flow Flow
	switch req.ResponseType {
	case "code":
		flow = FlowCode
	case "id_token":
		flow = FlowImplicit
	default:
		return "", throw(ErrAuthenticationErrUnsupportedResponseType, true)
	}

	if flow == FlowImplicit && req.Nonce == "" {
		return "", throw(ErrAuthenticationErrInvalidRequest, true)
	}

	if req.Prompt != LoginPromptUnspecified {
		return "", throw(errPromptIsUnsupported, false)
	}

	if req.MaxAge >= 0 {
		return "", throw(errMaxAgeIsUnsupported, false)
	}

	id, err := kvs.LoginSession_save(req.ClientId, req.RedirectURI, req.Scope, req.State, req.Nonce, int(flow))
	if err != nil {
		return "", throw(err, false)
	}

	return id, nil
}
