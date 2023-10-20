package auth

import (
	"database/sql"
	"time"

	"github.com/comame/accounts.comame.xyz/db"
)

type AuthenticationMethod string

var (
	AuthenticationMethodPassword AuthenticationMethod = "password"
)

func CreateAuthentication(
	tx *sql.Tx,
	aud, sub, userAgentID string,
	method AuthenticationMethod,
	authenticatedAt int64,
) error {
	now := time.Now().Unix()
	if err := db.Authentication_insertInTransaction(tx, aud, sub, userAgentID, string(method), now, authenticatedAt); err != nil {
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
