package auth

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"strings"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/timenow"
)

func CalculatePasswordHash(password string, salt string) string {
	withSalt := password + salt
	var bytes [32]byte

	for i := 0; i < 3; i += 1 {
		bytes = sha256.Sum256([]byte(withSalt))
	}

	return strings.ToUpper(hex.EncodeToString(bytes[:]))
}

func AuthenticateByPassword(ctx context.Context, sub, password, aud, userAgentID string) (bool, error) {
	con := db.Begin(ctx)
	defer con.Rollback()

	h := CalculatePasswordHash(password, sub)

	result, err := db.UserPassword_passwordMatchedInTransaction(con, sub, h)
	if err != nil {
		return false, err
	}

	now := timenow.Now().Unix()

	if err := CreateAuthentication(con, aud, sub, userAgentID, AuthenticationMethodPassword, now); err != nil {
		return false, err
	}

	if err := con.Commit(); err != nil {
		return false, err
	}

	return result != 0, nil
}
