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
