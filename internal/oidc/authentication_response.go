package oidc

import (
	"errors"
	"net/url"
)

type AuthenticationResponse struct {
	State   string
	Code    string
	IDToken string
	Error   string

	Flow        Flow
	RedirectURI string
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
