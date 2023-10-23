// 外部アカウント連携

package oidc

import (
	"net/url"

	"github.com/comame/accounts.comame.xyz/kvs"
	"github.com/comame/accounts.comame.xyz/random"
)

func GenerateGoogleAuthURL(loginSessionID, clientID, clientSecret string) (state, redirect string, err error) {
	_, err = kvs.LoginSession_get(loginSessionID)
	if err != nil {
		return "", "", err
	}

	state, err = random.String(32)
	if err != nil {
		return "", "", err
	}

	nonce, err := random.String(32)
	if err != nil {
		return "", "", err
	}

	if err := kvs.ExternalLoginSession_set(nonce, state, "google", loginSessionID); err != nil {
		return "", "", err
	}

	u, _ := url.Parse("https://accounts.google.com/o/oauth2/v2/auth")

	q := u.Query()
	q.Set("client_id", clientID)
	q.Set("response_type", "code")
	q.Set("scope", "openid email profile")
	q.Set("redirect_uri", "https://accounts.comame.xyz/oidc-callback/google")
	q.Set("state", state)
	q.Set("nonce", nonce)
	u.RawQuery = q.Encode()

	return state, u.String(), nil
}
