package oidc

import (
	"errors"
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
	panic("unreacahble")
}
