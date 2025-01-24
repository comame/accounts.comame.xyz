package ceremony

import (
	"errors"
	"time"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/random"
	"github.com/comame/accounts.comame.xyz/internal/timenow"
)

func createAccessToken(sub, scopes string) (token string, expiresIn int64, err error) {
	now := timenow.Now()

	t, err := random.String(32)
	if err != nil {
		return "", 0, err
	}

	createdAt := now.Format(db.DatetimeFormat)

	err = db.AccessToken_insert(t, sub, scopes, createdAt)
	if err != nil {
		return "", 0, err
	}

	// TODO: expiresIn を DB で表現できるようにする
	return t, 3600, nil
}

func findAccessToken(token string) (sub, scope string, err error) {
	sub, scope, createdAt, err := db.AccessToken_get(token)
	if err != nil {
		return "", "", err
	}

	now := timenow.Now()
	exp, err := time.Parse(db.DatetimeFormat, createdAt)
	if err != nil {
		return "", "", err
	}

	// TODO: expiresIn を DB で表現したい
	if now.After(exp.Add(3600 * time.Second)) {
		return "", "", errors.New("アクセストークンが失効している")
	}

	return sub, scope, nil
}
