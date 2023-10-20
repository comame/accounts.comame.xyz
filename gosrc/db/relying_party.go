package db

type relyingParty struct {
	ClientID           string
	HashedClientSecret string
}

func RelyingParty_select(clientID string) (*relyingParty, error) {
	con := Conn()
	row := con.QueryRow(`
		SELECT client_id, hashed_client_secret FROM relying_parties
		WHERE client_id = ?
	`, clientID)

	rp := new(relyingParty)
	if err := row.Scan(&rp.ClientID, &rp.HashedClientSecret); err != nil {
		return nil, err
	}

	return rp, nil
}

func RelyingParty_selectRedirectURIs(clientID string) ([]string, error) {
	con := Conn()
	rows, err := con.Query(`
		SELECT redirect_uri FROM redirect_uris
		WHERE client_id = ?
	`, clientID)
	if err != nil {
		return nil, err
	}

	var uris []string
	for rows.Next() {
		var uri string
		if err := rows.Scan(&uri); err != nil {
			return nil, err
		}
		uris = append(uris, uri)
	}

	return uris, nil
}
