package passkey

import (
	"encoding/base64"
	"errors"
)

// TODO: 全然ダミー実装

var keys = make(map[string]PublicKeyCredentialAttestation)

var (
	errBoundPublicKeyNotFound = errors.New("指定されたIDの公開鍵がユーザーに紐づけられていない")
)

func BindPublicKeyToUser(userID string, attestation PublicKeyCredentialAttestation) error {
	keys[attestation.ID] = attestation
	return nil
}

func FindPublicKey(userID string, assertion publicKeyCredentialAssertion) (*PublicKeyCredentialAttestation, error) {
	attestation, ok := keys[assertion.ID]
	if !ok {
		return nil, errBoundPublicKeyNotFound
	}

	return &attestation, nil
}

func userIDToUserHandle(userID string) string {
	return base64.RawURLEncoding.EncodeToString([]byte(userID))
}
