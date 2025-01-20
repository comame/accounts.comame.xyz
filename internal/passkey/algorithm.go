package passkey

import (
	"crypto"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"errors"
)

type algorithm int

var (
	algorithmRS256 algorithm = -257
	// algorithmEd25519 algorithm = -8
	// algorithmES256   algorithm = -7
)

var supportedAlgorithms = []publicKeyCredentialPubKeyCredParamsOptions{
	{
		// Windows, iOS
		Type: "public-key",
		Alg:  algorithmRS256,
	},
	// {
	// 	Type: "public-key",
	// 	Alg:  algorithmEd25519,
	// },
	// {
	// 	// Android
	// 	Type: "public-key",
	// 	Alg:  algorithmES256,
	// },
}

func isSupportedAlgorithm(alg algorithm) bool {
	for _, v := range supportedAlgorithms {
		if v.Alg == alg {
			return true
		}
	}

	return false
}

func parseRS256PublicKey(attestation *publicKeyCredentialAttestation) (*rsa.PublicKey, error) {
	bytes, err := base64.URLEncoding.DecodeString(attestation.Response.PublicKey)
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

func verifyRS256(attestation *publicKeyCredentialAttestation, payload, signature []byte) error {
	publicKey, err := parseRS256PublicKey(attestation)
	if err != nil {
		return err
	}

	hashed := sha256.Sum256(payload)
	if err := rsa.VerifyPKCS1v15(publicKey, crypto.SHA256, hashed[:], signature); err != nil {
		return err
	}

	return nil
}
