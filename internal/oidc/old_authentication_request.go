package oidc

import (
	"errors"
	"net/url"
	"strconv"
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

func ParseAuthenticationRequestFromQuery(q url.Values) (*AuthenticationRequest, error) {
	rawMaxAge := q.Get("max_age")
	maxAge, err := strconv.ParseInt(rawMaxAge, 0, 64)
	if err != nil && rawMaxAge != "" {
		return nil, errInvalidAuthenticationRequest
	}
	if err != nil && rawMaxAge == "" {
		maxAge = -1
	}

	req := AuthenticationRequest{
		Scope:        q.Get("scope"),
		ResponseType: q.Get("response_type"),
		ClientId:     q.Get("client_id"),
		RedirectURI:  q.Get("redirect_uri"),
		State:        q.Get("state"),
		Nonce:        q.Get("nonce"),
		Prompt:       LoginPrompt(q.Get("prompt")),
		MaxAge:       maxAge,
		IDTokenHint:  q.Get("id_token_hint"),
		Request:      q.Get("request"),
	}

	return &req, nil
}

func (r *AuthenticationRequest) Assert() error {
	if !r.Prompt.validate() {
		return errInvalidAuthenticationRequest
	}

	return nil
}
