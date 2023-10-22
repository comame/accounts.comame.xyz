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
