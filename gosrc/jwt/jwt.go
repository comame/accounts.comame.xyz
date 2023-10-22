package jwt

import (
	"crypto"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"errors"
	"math/big"
	"strings"
)

type Header struct {
	Typ string `json:"typ"`
	Alg string `json:"alg"`
}

type Payload struct {
	Iss   string `json:"iss"`
	Sub   string `json:"sub"`
	Aud   string `json:"aud"`
	Exp   int64  `json:"exp"`
	Iat   int64  `json:"iat"`
	Nonce string `json:"nonce"`

	// Custom claim
	Roles []string `json:"roles"`
}

type JWT struct {
	Header  Header
	Payload Payload
}

type JWKKey struct {
	N   string `json:"n"`
	E   string `json:"e"`
	Kty string `json:"kty"`
	Alg string `json:"alg"`
	Kid string `json:"kid"`
	Use string `json:"use"`
}

// BEGIN PUBLIC KEY で始まる pem をパースする
func DecodeRSAPrivKeyPem(pemString string) (*rsa.PrivateKey, error) {
	block, _ := pem.Decode([]byte(pemString))
	if block == nil {
		return nil, errors.New("invalid pem format")
	}

	var key *rsa.PrivateKey
	switch block.Type {
	case "RSA PRIVATE KEY":
		k, err := x509.ParsePKCS1PrivateKey(block.Bytes)
		if err != nil {
			return nil, err
		}
		key = k
	case "PRIVATE KEY":
		ki, err := x509.ParsePKCS8PrivateKey(block.Bytes)
		if err != nil {
			return nil, err
		}
		k, ok := ki.(*rsa.PrivateKey)
		if !ok {
			return nil, errors.New("invalid format rsa key")
		}
		key = k
	default:
		return nil, errors.New("invalid format")
	}

	key.Precompute()

	if err := key.Validate(); err != nil {
		return nil, err
	}

	return key, nil
}

func DecodeRSAPubKeyPem(pemString string) (*rsa.PublicKey, error) {
	block, _ := pem.Decode([]byte(pemString))

	if block.Type != "PUBLIC KEY" {
		return nil, errors.New("invalid pem format")
	}

	ki, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return nil, err
	}

	key, ok := ki.(*rsa.PublicKey)
	if !ok {
		return nil, errors.New("invalid RSA public key")
	}

	return key, nil
}

func EncodeJWT(header Header, payload Payload, privKey *rsa.PrivateKey) (string, error) {
	if header.Alg != "RS256" {
		return "", errors.New("unsupported alg")
	}

	hs, err := json.Marshal(header)
	if err != nil {
		return "", err
	}

	hb := base64.RawURLEncoding.EncodeToString(hs)

	ps, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	pb := base64.RawURLEncoding.EncodeToString(ps)

	hasher := sha256.New()
	hasher.Write([]byte(hb + "." + pb))

	s, err := rsa.SignPKCS1v15(nil, privKey, crypto.SHA256, hasher.Sum(nil))
	if err != nil {
		return "", err
	}

	sig := base64.RawURLEncoding.EncodeToString(s)

	return hb + "." + pb + "." + sig, nil
}

func DecodeJWT(jwt string, pubkey *rsa.PublicKey) (*Payload, error) {
	parts := strings.Split(jwt, ".")
	if len(parts) != 3 {
		return nil, errors.New("invalid jwt format")
	}

	bheader, err := base64.RawURLEncoding.DecodeString(parts[0])
	if err != nil {
		return nil, err
	}

	var header Header
	if err := json.Unmarshal(bheader, &header); err != nil {
		return nil, err
	}

	if header.Alg != "RS256" {
		return nil, errors.New("unsupported alg")
	}

	bpayload, err := base64.RawURLEncoding.DecodeString(parts[1])
	if err != nil {
		return nil, err
	}

	bsignature, err := base64.RawURLEncoding.DecodeString(parts[2])
	if err != nil {
		return nil, err
	}

	hasher := sha256.New()
	hasher.Write([]byte(parts[0] + "." + parts[1]))
	if err := rsa.VerifyPKCS1v15(pubkey, crypto.SHA256, hasher.Sum(nil), bsignature); err != nil {
		return nil, err
	}

	var payload Payload
	if err := json.Unmarshal(bpayload, &payload); err != nil {
		return nil, err
	}

	return &payload, nil
}

func DecodeJWK(key JWKKey) (*rsa.PublicKey, error) {
	if key.Alg != "RS256" || key.Kty != "RSA" {
		return nil, errors.New("unsupported alg")
	}

	nb, err := base64.RawURLEncoding.DecodeString(key.N)
	if err != nil {
		return nil, err
	}

	n := new(big.Int).SetBytes(nb)

	ne, err := base64.RawURLEncoding.DecodeString(key.E)
	if err != nil {
		return nil, err
	}

	e := int(new(big.Int).SetBytes(ne).Int64())

	return &rsa.PublicKey{
		N: n,
		E: e,
	}, nil

}

func EncodeToJWK(privKey *rsa.PublicKey, kid string) (*JWKKey, error) {
	if privKey.Size() != 256 {
		return nil, errors.New("unsupported alg")
	}

	k := &JWKKey{
		Kty: "RSA",
		Alg: "RSA256",
		Kid: kid,
		Use: "sig",
	}

	e := new(big.Int).SetInt64(int64(privKey.E))

	k.N = base64.RawURLEncoding.EncodeToString(privKey.N.Bytes())
	k.E = base64.RawURLEncoding.EncodeToString(e.Bytes())

	return k, nil
}
