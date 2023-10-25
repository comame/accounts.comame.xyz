package db

func AccessToken_insert(token, sub, scopes, createdAt string) error {
	con := Conn()

	if _, err := con.Exec(`
		INSERT INTO access_tokens
		(sub, scopes, token, created_at)
		VALUES
		(?, ?, ?, ?)
	`, sub, scopes, token, createdAt); err != nil {
		return err
	}

	return nil
}

func AccessToken_get(token string) (sub, scope, createdAt string, err error) {
	con := Conn()

	row := con.QueryRow(`
		SELECT sub, scopes, created_at FROM access_tokens
		WHERE token = ?
	`, token)

	err = row.Scan(&sub, &scope, &createdAt)
	if err != nil {
		return "", "", "", err
	}

	return sub, scope, createdAt, nil
}
