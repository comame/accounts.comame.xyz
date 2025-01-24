package ceremony

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"

	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

type signinGoogleAPIRequest struct {
	SessionID string `json:"state_id"`
}

var (
	googleClientID     = os.Getenv("GOOGLE_OIDC_CLIENT_ID")
	googleclientSecret = os.Getenv("GOOGLE_OIDC_CLIENT_ID")
	origin             = os.Getenv("HOST")
)

func StartGoogleSignin(w http.ResponseWriter, r io.Reader) {
	body, err := io.ReadAll(r)
	if err != nil {
		responseError(w, messageBadRequest)
		return
	}

	var request signinGoogleAPIRequest
	if err := json.Unmarshal(body, &request); err != nil {
		responseError(w, messageBadRequest)
		return
	}

	state, redirectURI, err := createGoogleAuthenticationURL(request.SessionID, googleClientID, origin)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "rp",
		Value:    state,
		MaxAge:   120,
		Secure:   true,
		HttpOnly: true,
		Path:     "/",
	})

	io.WriteString(w, fmt.Sprintf(`{ "location": "%s"}`, redirectURI))
}

func createGoogleAuthenticationURL(loginSessionID, clientID, myOrigin string) (state, redirectURL string, err error) {
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
	q.Set("redirect_uri", myOrigin+"/oidc-callback/google")
	q.Set("state", state)
	q.Set("nonce", nonce)
	u.RawQuery = q.Encode()

	return state, u.String(), nil
}
