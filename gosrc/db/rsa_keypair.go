package db

type rsaKeypair struct {
	Public  string
	Private string
	Kid     string
}

func RSAKeypair_get() (*rsaKeypair, error) {
	con := Conn()
	row := con.QueryRow(`
		SELECT public, private, kid FROM rsa_keypair
		WHERE id=1
	`)

	kp := new(rsaKeypair)
	if err := row.Scan(&kp.Public, &kp.Private, &kp.Kid); err != nil {
		return nil, err
	}
	return kp, nil
}

func RSAKeypair_delete() {
	con := Conn()
	con.Exec("DELETE FROM rsa_keypair")
}

func RSAKeypair_insertIgnore(public, private, kid string) error {
	con := Conn()
	if _, err := con.Exec(`
		INSERT IGNORE INTO rsa_keypair
		SET
			id='1',
			public=?,
			private=?,
			kid=?
	`, public, private, kid); err != nil {
		return err
	}
	return nil
}
