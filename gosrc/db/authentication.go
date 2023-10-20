package db

import "database/sql"

func Authentication_insertInTransaction(
	con *sql.Tx,
	aud, sub, userAgentID, method string,
	createdAt int64,
	authenticatedAt int64,
) error {
	if _, err := con.Exec(`
		INSERT INTO authentications
		(authenticated_at, created_at, audience, subject, user_agent_id, method)
		VALUES
		(?, ?, ?, ?, ?, ?)
	`, authenticatedAt, createdAt, aud, sub, userAgentID, method); err != nil {
		return err
	}
	return nil
}
