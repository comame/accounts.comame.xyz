package ceremony

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/cookie"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/random"
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
