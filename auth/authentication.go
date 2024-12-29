package auth

import (
	"database/sql"
	"time"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/timenow"
)

type AuthenticationMethod string

var (
	AuthenticationMethodPassword AuthenticationMethod = "password"
	AuthenticationMethodGoogle   AuthenticationMethod = "google"
)

func CreateAuthentication(
	tx *sql.Tx,
	aud, sub, userAgentID string,
	method AuthenticationMethod,
	authenticatedAt int64,
) error {
	now := timenow.Now().Format(db.DatetimeFormat)
	at := time.Unix(authenticatedAt, 0).Format(db.DatetimeFormat)
	if err := db.Authentication_insertInTransaction(tx, aud, sub, userAgentID, string(method), now, at); err != nil {
		return err
	}
	return nil
}

func Authorized(sub, clientId string) (bool, error) {
	count, err := db.RoleAccess_authorized(sub, clientId)
	if err != nil {
		return false, err
	}
	return count != 0, nil
}
