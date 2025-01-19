package passkey

import (
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"errors"
)

var supportedAlgorithms = []publicKeyCredentialPubKeyCredParamsOptions{
	{
		Type:    "public-key",
		Alg:     -257,
		algName: "RS256",
	},
	// {
	// 	Type:    "public-key",
	// 	Alg:     -8,
	// 	algName: "Ed25519",
	// },
	// {
	// 	Type: "public-key",
	// 	Alg:  -7,
	// 	algName: "ES256",
	// },
}

func getAlgFromNumber(num int) (alg string, ok bool) {
	for _, def := range supportedAlgorithms {
		if def.Alg == num {
			return def.algName, true
		}
	}
	return "", false
}

func parseRS256(derBase64 string) (*rsa.PublicKey, error) {
	bytes, err := base64.URLEncoding.DecodeString(derBase64)
	if err != nil {
		return nil, err
	}

	key, err := x509.ParsePKIXPublicKey(bytes)
	if err != nil {
		return nil, err
	}

	rsaPubKey, ok := key.(*rsa.PublicKey)
	if !ok {
		return nil, errors.New("RSA公開鍵ではなかった")
	}

	return rsaPubKey, nil
}
