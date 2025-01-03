package db

import (
	"database/sql"
	"errors"
)

func OpUser_get(sub, provider string) (string, bool, error) {
	con := Conn()

	row := con.QueryRow(`
		SELECT user_id FROM op_user
		WHERE op_user_id=? AND op=?
	`, sub, provider)

	var uid string
	if err := row.Scan(&uid); err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return "", false, nil
		}
		return "", false, err
	}

	return uid, true, nil
}
