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
