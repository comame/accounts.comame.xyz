package oidc

import (
	"errors"
	"fmt"
	"net/url"
	"strconv"
)

type AuthenticationRequest struct {
	Scope        string
	ResponseType string
	ClientId     string
	RedirectURI  string
	State        string
	Nonce        string
	Prompt       LoginPrompt
	// Negative MaxAge (-1) indicates unspecified.
	MaxAge      int64
	IDTokenHint string
	// unsupported parameter
	Request string
}

type LoginPrompt string

var (
	LoginPromptUnspecified   LoginPrompt = ""
	LoginPromptNone          LoginPrompt = "none"
	LoginPromptLogin         LoginPrompt = "login"
	LoginPromptConsent       LoginPrompt = "consent"
	LoginPromptSelectAccount LoginPrompt = "select_account"
)

type AuthenticationResponse struct {
	State   string
	Code    string
	IDToken string
	Error   string

	Flow        Flow
	RedirectURI string
}

func ParseAuthenticationRequest(v url.Values) (*AuthenticationRequest, error) {
	rawMaxAge := v.Get("max_age")
	maxAge, err := strconv.ParseInt(rawMaxAge, 0, 64)
	if err != nil && rawMaxAge != "" {
		return nil, fmt.Errorf("max_ageのパースに失敗 %s", rawMaxAge)
	}
	if err != nil && rawMaxAge == "" {
		maxAge = -1
	}

	req := AuthenticationRequest{
		Scope:        v.Get("scope"),
		ResponseType: v.Get("response_type"),
		ClientId:     v.Get("client_id"),
		RedirectURI:  v.Get("redirect_uri"),
		State:        v.Get("state"),
		Nonce:        v.Get("nonce"),
		Prompt:       LoginPrompt(v.Get("prompt")),
		MaxAge:       maxAge,
		IDTokenHint:  v.Get("id_token_hint"),
		Request:      v.Get("request"),
	}

	return &req, nil
}

func CreateRedirectURLFromAuthenticationResponse(res *AuthenticationResponse) (string, error) {
	u, err := url.Parse(res.RedirectURI)
	if err != nil {
		return "", err
	}

	q := make(url.Values)

	if res.State != "" {
		q.Set("state", res.State)
	}

	if res.Error != "" {
		q.Set("error", res.Error)

		// FIXME: ここの判定を enum ではなくする
		switch res.Flow {
		case FlowCode:
			u.RawQuery = q.Encode()
			return u.String(), nil
		case FlowImplicit:
			return u.String() + "#" + q.Encode(), nil
		default:
			return "", errors.New("invalid flow value")
		}
	}

	switch res.Flow {
	case FlowCode:
		q.Set("code", res.Code)
		u.RawQuery = q.Encode()
		return u.String(), nil
	case FlowImplicit:
		q.Set("id_token", res.IDToken)
		return u.String() + "#" + q.Encode(), nil
	default:
		return "", errors.New("invalid flow value")
	}
}
