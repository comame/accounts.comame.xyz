package passkey

import (
	"encoding/base64"
	"errors"
)

// TODO: 全然ダミー実装

var keys = make(map[string]publicKeyCredentialAttestation)

var (
	errBoundPublicKeyNotFound = errors.New("指定されたIDの公開鍵がユーザーに紐づけられていない")
)

func BindPublicKeyToUser(userID string, attestation publicKeyCredentialAttestation) error {
	keys[attestation.ID] = attestation
	return nil
}

func GetBoundPublicKey(userID string, assertion publicKeyCredentialAssertion) (*publicKeyCredentialAttestation, error) {
	attestation, ok := keys[assertion.ID]
	if !ok {
		return nil, errBoundPublicKeyNotFound
	}

	return &attestation, nil
}

func ListBoundKeyIDs(userID string) ([]string, error) {
	var ret []string
	for _, v := range keys {
		ret = append(ret, v.ID)
	}
	return ret, nil
}

func userIDToUserHandle(userID string) string {
	return base64.RawURLEncoding.EncodeToString([]byte(userID))
}
