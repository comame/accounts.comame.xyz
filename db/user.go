package db

func User_insert(id string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT INTO users
		SET id = ?
	`, id); err != nil {
		return err
	}
	return nil
}

func User_get(id string) (string, error) {
	con := Conn()
	row := con.QueryRow(`
		SELECT id FROM users WHERE id = ?
	`, id)

	var uid string
	if err := row.Scan(&uid); err != nil {
		return "", err
	}
	return uid, nil
}
