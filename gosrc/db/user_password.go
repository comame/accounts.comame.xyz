package db

import (
	"database/sql"
)

func UserPassword_passwordMatchedInTransaction(con *sql.Tx, user, passwordHash string) (int, error) {
	row := con.QueryRow(`
		SELECT count(*) FROM user_passwords
		WHERE
			user_id = ? AND hashed_password = ?
	`, user, passwordHash)

	var count int
	if err := row.Scan(&count); err != nil {
		return 0, err
	}
	return count, nil
}
