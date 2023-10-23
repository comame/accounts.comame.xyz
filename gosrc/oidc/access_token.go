package oidc

import (
	"time"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/random"
)

func CreateAccessToken(sub, scopes string) (token string, expiresIn int64, err error) {
	now := time.Now()

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
