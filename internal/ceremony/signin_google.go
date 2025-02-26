package ceremony

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/cookie"
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/httpclient"
	"github.com/comame/accounts.comame.xyz/internal/jwt"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
	"github.com/comame/accounts.comame.xyz/internal/random"
	"github.com/comame/accounts.comame.xyz/internal/timenow"
)

type signinGoogleAPIRequest struct {
	SessionID string `json:"state_id"`
}

var (
	googleClientID     = os.Getenv("GOOGLE_OIDC_CLIENT_ID")
	googleClientSecret = os.Getenv("GOOGLE_OIDC_CLIENT_SECRET")
	origin             = os.Getenv("HOST")
)

func StartGoogleSignin(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		responseError(w, messageBadRequest)
		return
	}

	var request signinGoogleAPIRequest
	if err := json.Unmarshal(body, &request); err != nil {
		responseError(w, messageBadRequest)
		return
	}

	loginSessionCookie, err := r.Cookie(cookie.LoginSession)
	if err != nil {
		log.Println(err)
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}
	loginSessionID := loginSessionCookie.Value

	state, redirectURI, err := createGoogleAuthenticationURL(loginSessionID, googleClientID, origin)
	if err != nil {
		// TODO: 当然ながらここでエラーを吐くので、直す
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	// FIXME: Cookie に直接 state を保存していて危険
	http.SetCookie(w, &http.Cookie{
		Name:     cookie.ExternalLoginSession,
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

func HandleCallbackFromGoogle(w http.ResponseWriter, r *http.Request) {
	query := r.URL.Query()

	googleState := query.Get("state")
	googleCode := query.Get("code")

	if googleState == "" {
		log.Println("Googleからのコールバックに state がない")
		responseError(w, messageBadRequest)
		return
	}
	if googleCode == "" {
		log.Println("Googleからのコールバックに code がない")
		responseError(w, messageBadRequest)
		return
	}

	cookieState, err := r.Cookie(cookie.ExternalLoginSession)
	if err != nil {
		log.Println("セッションに紐づけられたstateがない")
		responseError(w, messageBadRequest)
		return
	}
	if cookieState.Value != googleState {
		responseError(w, messageBadRequest)
		return
	}

	userID, clientID, err := callbackGoogleInternal(googleCode, googleState, googleClientID, googleClientSecret, origin)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	if err := markSigninSessionAuthenticated(r, userID); err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	// TODO: signin?g=1にリダイレクト
	q := make(url.Values)
	q.Set("g", "1")
	q.Set("cid", clientID)

	w.Header().Set("Location", "/signin?"+q.Encode())
	w.WriteHeader(http.StatusSeeOther)
}

func callbackGoogleInternal(googleCode, googleState, googleClientID, googleClientSecret, myOrigin string) (userID, clientID string, err error) {
	saved, err := kvs.ExternalLoginSession_get(googleState)
	if err != nil {
		return "", "", err
	}
	kvs.ExternalLoginSession_delete(googleState)

	if saved.Provider != "google" {
		return "", "", errors.New("google ログインを要求していないのに Authentication Response を受け取った")
	}

	session, err := kvs.LoginSession_get(saved.LoginSession)
	if err != nil {
		return "", "", err
	}

	codeRes, err := doGoogleCodeRequest(googleCode, googleClientID, googleClientSecret, myOrigin)
	if err != nil {
		return "", "", err
	}

	tokenHeader, err := jwt.ExtractJWTHeader(codeRes.IDToken)
	if err != nil {
		return "", "", err
	}

	keys, err := getGoogleKeys()
	if err != nil {
		return "", "", err
	}

	var key *jwt.JWKKey
	for _, v := range keys {
		if v.Kid == tokenHeader.Kid {
			key = &v
			break
		}
	}
	if key == nil {
		return "", "", errors.New("id_token を署名した公開鍵が Google の Certs に存在しない")
	}

	keypub, err := jwt.DecodeJWK(*key)
	if err != nil {
		return "", "", err
	}

	claim, err := jwt.DecodeJWT(codeRes.IDToken, keypub)
	if err != nil {
		return "", "", err
	}

	now := timenow.Now().Unix()

	if claim.Iss != "https://accounts.google.com" {
		return "", "", errors.New("iss が Google でない")
	}

	if claim.Exp < now {
		return "", "", errors.New("id_token が失効している")
	}

	if now < claim.Iat {
		return "", "", errors.New("iat が未来")
	}

	if claim.Aud != googleClientID {
		return "", "", errors.New("aud が自分ではない")
	}

	if claim.Nonce != saved.Nonce {
		return "", "", errors.New("nonce が異なる")
	}

	var sub string

	linkedSub, isLinked, err := db.OpUser_get(claim.Sub, "google")
	if err != nil {
		return "", "", err
	}
	if isLinked {
		sub = linkedSub
	} else {
		sub = claim.Sub + "@accounts.google.com"

		// 外部連携の場合、暗黙的にアカウントを作成する
		_, err := db.User_get(sub)
		if err != nil && !errors.Is(err, sql.ErrNoRows) {
			return "", "", err
		}
		if err != nil {
			if err := createGoogleUser(sub); err != nil {
				return "", "", err
			}
		}
	}

	// TODO: userinfo を取る

	roleOk, err := auth.Authorized(sub, session.RelyingPartyID)
	if err != nil {
		return "", "", err
	}
	if !roleOk {
		return "", "", errors.New("権限がない")
	}

	return sub, session.RelyingPartyID, nil
}

func doGoogleCodeRequest(code, clientID, clientSecret, myOrigin string) (*oidc.CodeResponse, error) {
	q := make(url.Values)
	q.Set("client_id", clientID)
	q.Set("client_secret", clientSecret)
	q.Set("grant_type", "authorization_code")
	q.Set("code", code)
	q.Set("redirect_uri", myOrigin+"/oidc-callback/google")

	bod := q.Encode()
	buf := bytes.NewBufferString(bod)

	req, err := http.NewRequest(http.MethodPost, "https://oauth2.googleapis.com/token", buf)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	res, err := httpclient.Client().Do(req)
	if err != nil {
		return nil, err
	}

	resb, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	var cres oidc.CodeResponse
	if err := json.Unmarshal(resb, &cres); err != nil {
		return nil, err
	}

	return &cres, nil
}

func getGoogleKeys() ([]jwt.JWKKey, error) {
	res, err := httpclient.Client().Get("https://www.googleapis.com/oauth2/v3/certs")
	if err != nil {
		return nil, err
	}

	resb, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	var keys jwt.JWK
	if err := json.Unmarshal(resb, &keys); err != nil {
		return nil, err
	}

	return keys.Keys, nil
}

func createGoogleUser(sub string) error {
	if err := db.User_insert(sub); err != nil {
		return err
	}
	if err := db.UserRole_insert(sub, "google"); err != nil {
		return err
	}
	if err := db.UserRole_insert(sub, "everyone"); err != nil {
		return err
	}
	return nil
}
