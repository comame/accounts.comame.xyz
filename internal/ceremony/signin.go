package ceremony

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/cookie"
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/jwt"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
	"github.com/comame/accounts.comame.xyz/internal/random"
	"github.com/comame/accounts.comame.xyz/internal/timenow"
)

var (
	messageBadRequest        = "bad_request"
	messageInvalidCredential = "invalid_credential"
	messageUnauthorized      = "unauthorized"
)

func responseError(w http.ResponseWriter, message string) {
	w.WriteHeader(http.StatusBadRequest)
	io.WriteString(w, fmt.Sprintf(`{ "error": "%s" }`, message))
}

// 認証セッションを表す
// Authentication Request で開始され、ユーザーが認証されたら認証状態に変化し、RPにレスポンスを返したら終了する
type signinSession struct {
	ID            string
	UserID        string
	Authenticated bool
}

// 未認証の認証セッションを作成する
func createEmptySigninSession(w http.ResponseWriter) error {
	id, err := random.String(32)
	if err != nil {
		return errors.Join(errors.New("乱数生成に失敗"), err)
	}

	v := signinSession{
		ID:            id,
		Authenticated: false,
	}
	if err := kvs.Set2(context.Background(), kvs.KeySigninSession(id), v, 10*60); err != nil {
		return err
	}

	http.SetCookie(w, &http.Cookie{
		Name:     cookie.SigninSession,
		Value:    id,
		MaxAge:   10 * 60,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteLaxMode,
	})

	return nil
}

func markSigninSessionAuthenticated(r *http.Request, userID string) error {
	cookie, err := r.Cookie(cookie.SigninSession)
	if err != nil {
		return err
	}

	if cookie.Value == "" {
		return errors.New("SigninSessionのキーが空文字列")
	}

	session, err := kvs.Get2[signinSession](context.Background(), kvs.KeySigninSession(cookie.Value))
	if err != nil {
		return err
	}

	if session.ID != cookie.Value {
		return errors.New("KVSに保存されていたIDとSigninSession.IDが異なる")
	}
	if session.Authenticated {
		return errors.New("すでに認証されているセッションに対して、再び認証状態に変化させようとした")
	}

	session.Authenticated = true
	session.UserID = userID
	if err := kvs.Set2(context.Background(), kvs.KeySigninSession(session.ID), session, 3*60); err != nil {
		return err
	}

	return nil
}

func getUserIDFromSigninSession(r *http.Request) (string, error) {
	cookie, err := r.Cookie(cookie.SigninSession)
	if err != nil {
		return "", err
	}

	if cookie.Value == "" {
		return "", errors.New("SigninSessionのキーが空文字列")
	}

	session, err := kvs.Get2[signinSession](context.Background(), kvs.KeySigninSession(cookie.Value))
	if err != nil {
		return "", err
	}
	kvs.Del(context.Background(), kvs.KeySigninSession(cookie.Value))

	if !session.Authenticated {
		return "", errors.New("未認証のセッション")
	}

	return session.UserID, nil
}

// IDToken を発行する。sub が確定したのち呼ぶ。
func createAuthenticationResponse(sub, stateID, aud string) (*oidc.AuthenticationResponse, error) {
	authorized, err := auth.Authorized(sub, aud)
	if err != nil {
		return nil, err
	}
	if !authorized {
		return nil, errors.New("ロールがない")
	}

	state, err := kvs.LoginSession_get(stateID)
	if err != nil {
		return nil, err
	}
	kvs.LoginSession_delete(stateID)

	// TODO: userAgentID を使う？

	if state.RelyingPartyID != aud {
		return nil, errors.New("RelyingPartyID mismatch")
	}

	roles, err := db.Role_getUserRoles(sub)
	if err != nil {
		return nil, err
	}

	now := timenow.Now().Unix()

	// TODO: auth_at に対応する
	// TODO: email, email_verified, name, preferred_username, profile, picture に対応する
	claim := jwt.Payload{
		Iss:   "https://accounts.comame.xyz",
		Sub:   sub,
		Aud:   state.RelyingPartyID,
		Iat:   now,
		Exp:   now + int64(5*60),
		Roles: roles,
	}
	if state.Nonce != "" {
		claim.Nonce = state.Nonce
	}

	kp, err := db.RSAKeypair_get()
	if err != nil {
		return nil, err
	}
	priv, err := jwt.DecodeRSAPrivKeyPem(kp.Private)
	if err != nil {
		return nil, err
	}
	token, err := jwt.EncodeJWT(jwt.Header{
		Alg: "RS256",
		Typ: "JWT",
		Kid: kp.Kid,
	}, claim, priv)
	if err != nil {
		return nil, err
	}

	switch oidc.Flow(state.Flow) {
	case oidc.FlowCode:
		code, err := random.String(32)
		if err != nil {
			return nil, err
		}
		if err := kvs.CodeState_save(code, aud, sub, token, state.Scopes, state.RedirectURI); err != nil {
			return nil, err
		}
		res := &oidc.AuthenticationResponse{
			Code:        code,
			Flow:        oidc.FlowCode,
			RedirectURI: state.RedirectURI,
		}
		if state.State != "" {
			res.State = state.State
		}
		return res, nil
	case oidc.FlowImplicit:
		res := &oidc.AuthenticationResponse{
			IDToken:     token,
			Flow:        oidc.FlowImplicit,
			RedirectURI: state.RedirectURI,
		}
		if state.State != "" {
			res.State = state.State
		}
		return res, nil
	default:
		return nil, errors.New("invalid state")
	}
}
