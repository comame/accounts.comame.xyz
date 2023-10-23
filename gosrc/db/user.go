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
