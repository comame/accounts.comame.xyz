package passkey

import (
	"encoding/base64"
	"errors"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

// challenge を生成してセッションに紐づける
func CreateChallengeAndBindSession(userID string, w http.ResponseWriter) ([]byte, error) {
	buf := make([]byte, 32)
	n, err := random.Bytes(buf)
	if err != nil {
		return nil, err
	}
	if n != 32 {
		return nil, errors.New("challengeが32バイトではない")
	}

	str := base64.RawURLEncoding.EncodeToString(buf)

	if err := kvs.PasskeyChallenge_save(str, userID); err != nil {
		return nil, err
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "pkc",
		Value:    str, // FIXME: チャレンジを直接Cookieに保存しているが、これはとても危険
		MaxAge:   10 * 60,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
	})

	return buf, nil
}

// セッションに紐づけられたchallengeを取得する
func GetChallengeFromSession(userID string, r *http.Request) ([]byte, error) {
	cookie, err := r.Cookie("pkc")
	if err != nil {
		return nil, err
	}

	str := cookie.Value
	buf, err := base64.RawURLEncoding.DecodeString(str)
	if err != nil {
		return nil, err
	}

	defer kvs.PasskeyChallenge_delete(str)
	kvsUserID, err := kvs.PasskeyChallenge_get(str)
	if err != nil {
		return nil, err
	}
	if kvsUserID != userID {
		return nil, errors.New("パスキー登録時にkvsに保存されたuserIDがセッションと異なる")
	}

	return buf, nil
}
