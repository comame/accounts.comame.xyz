// 外部アカウント連携

package oidc

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"net/url"
	"time"

	"github.com/comame/accounts.comame.xyz/auth"
	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/jwt"
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

func CallbackGoogle(code, state, clientID, clientSecret string) (*AuthenticationResponse, error) {
	saved, err := kvs.ExternalLoginSession_get(state)
	if err != nil {
		return nil, err
	}
	kvs.ExternalLoginSession_delete(state)

	if saved.Provider != "google" {
		return nil, errors.New("google ログインを要求していないのに Authentication Response を受け取った")
	}

	session, err := kvs.LoginSession_get(saved.LoginSession)
	if err != nil {
		return nil, err
	}

	codeRes, err := doGoogleCodeRequest(code, clientID, clientSecret)
	if err != nil {
		return nil, err
	}

	tokenHeader, err := jwt.ExtractJWTHeader(codeRes.IDToken)
	if err != nil {
		return nil, err
	}

	keys, err := getGoogleKeys()
	if err != nil {
		return nil, err
	}

	var key *jwt.JWKKey
	for _, v := range keys {
		if v.Kid != tokenHeader.Kid {
			continue
		}
		key = &v
	}
	if key == nil {
		return nil, errors.New("id_token を署名した公開鍵が Google の Certs に存在しない")
	}

	keypub, err := jwt.DecodeJWK(*key)
	if err != nil {
		return nil, err
	}

	claim, err := jwt.DecodeJWT(codeRes.IDToken, keypub)
	if err != nil {
		return nil, err
	}

	now := time.Now().Unix()

	if claim.Iss != "https://accounts.google.com" {
		return nil, errors.New("iss が Google でない")
	}

	if claim.Exp < now {
		return nil, errors.New("id_token が失効している")
	}

	if now < claim.Iat {
		return nil, errors.New("iat が未来")
	}

	if claim.Aud != clientID {
		return nil, errors.New("aud が自分ではない")
	}

	if claim.Nonce != saved.Nonce {
		return nil, errors.New("nonce が異なる")
	}

	var sub string

	linkedSub, isLinked, err := db.OpUser_get(claim.Sub, "google")
	if err != nil {
		return nil, err
	}
	if isLinked {
		sub = linkedSub
	} else {
		sub = claim.Sub + "@accounts.google.com"

		// 外部連携の場合、暗黙的にアカウントを作成する
		_, err := db.User_get(sub)
		if err != nil && !errors.Is(err, sql.ErrNoRows) {
			return nil, err
		}
		if err != nil {
			if err := createGoogleUser(sub); err != nil {
				return nil, err
			}
		}
	}

	// TODO: userinfo を取る

	roleOk, err := auth.Authorized(sub, session.RelyingPartyID)
	if err != nil {
		return nil, err
	}
	if !roleOk {
		return nil, errors.New("権限がない")
	}

	// TODO: user_agent_id を消す
	res, err := PostAuthentication(sub, saved.LoginSession, session.RelyingPartyID, "", auth.AuthenticationMethodGoogle)
	if err != nil {
		return nil, err
	}
	return res, nil
}

func doGoogleCodeRequest(code, clientID, clientSecret string) (*codeResponse, error) {
	q := make(url.Values)
	q.Set("client_id", clientID)
	q.Set("client_secret", clientSecret)
	q.Set("grant_type", "authorization_code")
	q.Set("code", code)
	q.Set("redirect_uri", "https://accounts.comame.xyz/oidc-callback/google")

	bod := q.Encode()
	buf := bytes.NewBufferString(bod)

	req, err := http.NewRequest(http.MethodPost, "https://oauth2.googleapis.com/token", buf)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}

	resb, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	var cres codeResponse
	if err := json.Unmarshal(resb, &cres); err != nil {
		return nil, err
	}

	return &cres, nil
}

func getGoogleKeys() ([]jwt.JWKKey, error) {
	res, err := http.Get("https://www.googleapis.com/oauth2/v3/certs")
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
