package passkey

import (
	"encoding/base64"
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

	challengeStr := base64.RawURLEncoding.EncodeToString(challengeBytes)

	sessionID, err := random.String(32)
	if err != nil {
		return nil, err
	}

	if err := kvs.PasskeyChallenge_save(challengeStr, sessionID); err != nil {
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

	defer kvs.PasskeyChallenge_delete(sessionID)
	challengeString, err := kvs.PasskeyChallenge_get(sessionID)
	if err != nil {
		return nil, err
	}

	buf, err := base64.RawURLEncoding.DecodeString(challengeString)
	if err != nil {
		return nil, err
	}

	return buf, nil
}
