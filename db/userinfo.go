package db

func UserInfo_get(sub string) (string, error) {
	con := Conn()

	row := con.QueryRow(`
		SELECT value
		FROM userinfo
		WHERE sub=?
	`, sub)

	var v string
	if err := row.Scan(&v); err != nil {
		return "", err
	}

	return v, nil
}

func UserInfo_insert(sub, value string) error {
	con := Conn()

	if _, err := con.Exec(`
		INSERT INTO userinfo SET sub=?, value=?
	`, sub, value); err != nil {
		return err
	}
	return nil
}
