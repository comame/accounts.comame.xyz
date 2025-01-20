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

// 指定したユーザーに公開鍵を紐づける
func BindPublicKeyToUser(userID string, attestation publicKeyCredentialAttestation) error {
	keys[attestation.ID] = attestation
	return nil
}

// PublicKeyCredentialAssertion に紐づく公開鍵を探す
func GetBoundPublicKey(userID string, assertion publicKeyCredentialAssertion) (*publicKeyCredentialAttestation, error) {
	attestation, ok := keys[assertion.ID]
	if !ok {
		return nil, errBoundPublicKeyNotFound
	}

	return &attestation, nil
}

// アカウントに紐づけられた公開鍵IDのリストを返す
func ListBoundKeyIDs(userID string) ([]string, error) {
	var ret []string
	for _, v := range keys {
		ret = append(ret, v.ID)
	}
	return ret, nil
}

func ConvertUserIDToUserHandle(userID string) string {
	return base64.RawURLEncoding.EncodeToString([]byte(userID))
}

// PublicKeyCredentialAssertion から userID を **検証せずに** 取り出す
func AssumeUserID(assertion *publicKeyCredentialAssertion) (string, error) {
	b, err := base64.RawURLEncoding.DecodeString(assertion.Response.UserHandle)
	if err != nil {
		return "", err
	}
	return string(b), nil
}
