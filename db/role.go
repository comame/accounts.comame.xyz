package db

func Role_insert(name string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT INTO role
		SET name=?
	`, name); err != nil {
		return err
	}
	return nil
}

func Role_getUserRoles(user string) ([]string, error) {
	con := Conn()

	rows, err := con.Query("SELECT role FROM user_role WHERE user_id=?", user)
	if err != nil {
		return nil, err
	}

	var roles []string
	for rows.Next() {
		var r string
		if err := rows.Scan(&r); err != nil {
			return nil, err
		}
		roles = append(roles, r)
	}

	return roles, nil
}
