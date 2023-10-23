package scripts

import (
	"bytes"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"

	"github.com/comame/accounts.comame.xyz/auth"
	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/random"
)

const scriptSetupDefault = "setup_default"

func init() {
	register(scriptSetupDefault, setupDefault, "adminPassword, dashboardClientSecret")
}

func setupDefault(args ...string) error {
	adminPassword := args[0]
	dashboardClientSecret := args[1]

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

	if err := db.RSAKeypair_insertIgnore(pubPem, privPem, kid); err != nil {
		return err
	}

	if err := db.User_insert("admin"); err != nil {
		return err
	}

	ph := auth.CalculatePasswordHash(adminPassword, "admin")
	if err := db.UserPassword_insert("admin", ph); err != nil {
		return err
	}

	if err := db.Role_insert("admin"); err != nil {
		return err
	}
	if err := db.Role_insert("everyone"); err != nil {
		return err
	}

	if err := db.UserRole_insert("admin", "admin"); err != nil {
		return err
	}

	cs := auth.CalculatePasswordHash(dashboardClientSecret, "accounts.comame.xyz")
	if err := db.RelyingParty_insert("accounts.comame.xyz", cs); err != nil {
		return err
	}

	if err := db.RoleAccess_insert("admin", "accounts.comame.xyz"); err != nil {
		return err
	}

	if err := db.RelyingParty_newRedirectURI("accounts.comame.xyz", "https://dash.accounts.comame.xyz/callback"); err != nil {
		return err
	}

	return nil
}

func setupDefault_generateKeypair() (*rsa.PrivateKey, *rsa.PublicKey, error) {
	priv, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, nil, err
	}

	if err := priv.Validate(); err != nil {
		return nil, nil, err
	}

	return priv, &priv.PublicKey, nil
}

func setupDefault_pemPubkey(pubkey *rsa.PublicKey) (string, error) {
	der, err := x509.MarshalPKIXPublicKey(pubkey)
	if err != nil {
		return "", err
	}

	b := &pem.Block{
		Type:  "PUBLIC KEY",
		Bytes: der,
	}

	w := bytes.NewBuffer(nil)
	if err := pem.Encode(w, b); err != nil {
		return "", err
	}

	return w.String(), nil
}

func setupDefault_pemPrivkey(privkey *rsa.PrivateKey) (string, error) {
	der := x509.MarshalPKCS1PrivateKey(privkey)
	b := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: der,
	}

	w := bytes.NewBuffer(nil)
	if err := pem.Encode(w, b); err != nil {
		return "", err
	}

	return w.String(), nil
}
