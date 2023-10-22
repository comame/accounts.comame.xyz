package oidc

import (
	"encoding/json"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/jwt"
)

func GetDiscoveryCertsJSON() ([]byte, error) {
	keypair, err := db.RSAKeypair_get()
	if err != nil {
		return nil, err
	}

	k, err := jwt.DecodeRSAPubKeyPem(keypair.Public)
	if err != nil {
		return nil, err
	}

	key, err := jwt.EncodeToJWK(k, keypair.Kid)
	if err != nil {
		return nil, err
	}

	jwk := jwt.JWK{
		Keys: []jwt.JWKKey{*key},
	}

	js, err := json.MarshalIndent(jwk, "", "    ")
	if err != nil {
		return nil, err
	}

	return js, nil
}
