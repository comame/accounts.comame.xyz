package oidc

import (
	"errors"
)

var (
	errInvalidAuthenticationRequest = errors.New("invalid authentication request")
)

func (p LoginPrompt) validate() bool {
	return p == LoginPromptUnspecified ||
		p == LoginPromptNone ||
		p == LoginPromptLogin ||
		p == LoginPromptConsent ||
		p == LoginPromptSelectAccount
}

func (r *AuthenticationRequest) Assert() error {
	if !r.Prompt.validate() {
		return errInvalidAuthenticationRequest
	}

	return nil
}
