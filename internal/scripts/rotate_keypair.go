package scripts

import (
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

func init() {
	register("rotate_keypair", RotateKeypair, "")
}

func RotateKeypair(args ...string) error {
	priv, pub, err := setupDefault_generateKeypair()
	if err != nil {
		return err
	}
	privPem, err := setupDefault_pemPrivkey(priv)
	if err != nil {
		return err
	}
	pubPem, err := setupDefault_pemPubkey(pub)
	if err != nil {
		return err
	}

	kid, err := random.String(8)
	if err != nil {
		return err
	}

	db.RSAKeypair_delete()

	if err := db.RSAKeypair_insertIgnore(pubPem, privPem, kid); err != nil {
		return err
	}

	return nil
}
