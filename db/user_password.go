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

func UserPassword_insert(user, passwordHash string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT INTO user_passwords
		SET
			user_id = ?, hashed_password = ?
	`, user, passwordHash); err != nil {
		return err
	}
	return nil
}
