package passkey

import (
	"crypto"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
)

func GetOptions(rpID string, challenge []byte) credentialGetOptions {
	challengeBase64 := base64.RawURLEncoding.EncodeToString(challenge)

	return credentialGetOptions{
		PublicKey: publicKeyCredentialRequestOptions{
			ChallengeBase64:  challengeBase64,
			RPID:             rpID,
			UserVerification: "required",
		},
	}
}

func ParseAssertion(jsonReader io.Reader, challenge []byte, origin string) (*publicKeyCredentialAssertion, error) {
	bytes, err := io.ReadAll(jsonReader)
	if err != nil {
		return nil, err
	}

	var assertion publicKeyCredentialAssertion
	if err := json.Unmarshal(bytes, &assertion); err != nil {
		return nil, err
	}

	if assertion.ID == "" || assertion.RawID == "" {
		return nil, errors.New("assertionのIDが空だった")
	}

	if assertion.ID != assertion.RawID {
		return nil, errors.New("assertionのIDとrawIDが一致しなかった")
	}

	clientDataJSON, err := base64.RawStdEncoding.DecodeString(assertion.Response.ClientDataJSON)
	if err != nil {
		return nil, errors.Join(errors.New("assertionのclientDataJSONのパースに失敗した"), err)
	}

	var clientData authenticatorResponseClientData
	if err := json.Unmarshal(clientDataJSON, &clientData); err != nil {
		return nil, errors.Join(errors.New("assertionのclientDataのパースに失敗した"), err)
	}

	if clientData.Type != "webauthn.get" {
		return nil, fmt.Errorf("assertionのclientData.typeが未知 %s", clientData.Type)
	}

	if clientData.Origin != origin {
		return nil, fmt.Errorf("assertionのclientData.originが不正 %s", clientData.Origin)
	}

	if clientData.CrossOrigin {
		return nil, fmt.Errorf("assertionのclientData.crossOrigin=trueが不正")
	}

	assertion.Response.clientData = clientData

	return &assertion, nil
}

func VerifyAssertion(userID string, attestation publicKeyCredentialAttestation, assertion publicKeyCredentialAssertion) error {
	if attestation.ID != assertion.ID {
		return errors.New("パスキーの検証時に keyId が一致しなかった")
	}

	if assertion.Response.UserHandle != userIDToUserHandle(userID) {
		return errors.New("パスキーの検証時に userHandle と userID が一致しなかった")
	}

	// 署名検証にかかわるステップ
	// 8. Let cData, authData and sig denote the value of response’s clientDataJSON, authenticatorData, and signature respectively.
	// 19. Let hash be the result of computing a hash over the cData using SHA-256.
	// 20. Using credentialPublicKey, verify that sig is a valid signature over the binary concatenation of authData and hash.

	hash := sha256.Sum256([]byte(assertion.Response.ClientDataJSON))
	publicKey, err := publicKeyRS256(&attestation)
	if err != nil {
		return errors.Join(errors.New("パスキーの検証時にattestationから公開鍵を取り出せなかった"), err)
	}

	decodedAuthData, err := base64.RawURLEncoding.DecodeString(assertion.Response.AuthenticatorData)
	if err != nil {
		return errors.Join(errors.New("パスキーの検証時にauthenticatorDataのデコードに失敗した"), err)
	}
	decodedSig, err := base64.RawURLEncoding.DecodeString(assertion.Response.Signature)
	if err != nil {
		return errors.Join(errors.New("パスキーの検証時にsignatureのデコードに失敗した"), err)
	}

	var hashedMessage []byte
	hashedMessage = append(hashedMessage, decodedAuthData...)
	hashedMessage = append(hashedMessage, hash[:]...)
	if err := rsa.VerifyPKCS1v15(publicKey, crypto.SHA256, hashedMessage, decodedSig); err != nil {
		return errors.Join(errors.New("パスキーの検証時に署名の検証に失敗した"), err)
	}

	// TODO: Step 21 該当の公開鍵が使われた回数に応じてリスク判定を行う

	return nil
}
