package passkey

import (
	"context"
	"errors"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

// challenge を生成してセッションに紐づける
func CreateChallengeAndBindSession(w http.ResponseWriter) (challenge []byte, err error) {
	challengeBytes := make([]byte, 32)
	n, err := random.Bytes(challengeBytes)
	if err != nil {
		return nil, err
	}
	if n != 32 {
		return nil, errors.New("challengeが32バイトではない")
	}

	sessionID, err := random.String(32)
	if err != nil {
		return nil, err
	}

	if err := kvs.Set2(context.Background(), kvs.KeyPasskeyChallenge(sessionID), challengeBytes, 10*60); err != nil {
		return nil, err
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "pkc",
		Value:    sessionID,
		MaxAge:   10 * 60,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
	})

	return challengeBytes, nil
}

// セッションに紐づけられたchallengeを取得する
func GetChallengeFromSession(w http.ResponseWriter, r *http.Request) (challenge []byte, err error) {
	cookie, err := r.Cookie("pkc")
	if err != nil {
		return nil, err
	}

	sessionID := cookie.Value
	if sessionID == "" {
		return nil, errors.New("challengeのセッションIDが空")
	}

	defer kvs.Del(context.Background(), kvs.KeyPasskeyChallenge(sessionID))
	buf, err := kvs.Get2[[]byte](r.Context(), kvs.KeyPasskeyChallenge(sessionID))
	if err != nil {
		return nil, err
	}

	return buf, nil
}
