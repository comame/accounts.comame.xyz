package db

func UserRole_insert(user, role string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT INTO user_role
		SET user_id = ?, role = ?
	`, user, role); err != nil {
		return err
	}
	return nil
}
