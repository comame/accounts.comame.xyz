package db

func RoleAccess_authorized(userID, relyingPartyID string) (int, error) {
	con := Conn()

	row := con.QueryRow(`
		SELECT COUNT(*) FROM user_role
		INNER JOIN role_access ON
			user_role.role = role_access.role
		WHERE user_id = ? AND relying_party_id = ?
	`, userID, relyingPartyID)

	var count int
	if err := row.Scan(&count); err != nil {
		return 0, err
	}
	return count, nil
}

func RoleAccess_insert(role, relyingParty string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT INTO role_access SET role=?, relying_party_id=?
	`, role, relyingParty); err != nil {
		return err
	}
	return nil
}
