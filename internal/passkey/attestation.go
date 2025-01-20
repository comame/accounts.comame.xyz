package passkey

import (
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
)

// navigator.credentials.create に渡す CredentialCreationOptions を生成する
func CreateOptions(rpID, rpName string, userID string, userName, userDisplayName string, excludeKeyIDs []string, challenge []byte) credentialCreateOptions {
	userIDBase64 := userIDToUserHandle(userID)
	challengeBase64 := base64.RawURLEncoding.EncodeToString(challenge)

	opt := credentialCreateOptions{
		PublicKey: publicKeyCredentialCreationOptions{
			ChallengeBase64: challengeBase64,
			AuthenticatorSelection: publicKeyCredentialAuthenticatorSelectionOptions{
				// 登録できるのはその端末の認証機だけとする (他デバイスは Hybrid Transport でログインすればよい)
				AuthenticatorAttachment: "platform",
				// ユーザー名入力なしでログインしたいので、Discoverable Credential のみ受け付ける
				RequireResidentKey: true,
				ResidentKey:        "required",
				// UserVerification は求める
				UserVerification: "required",
			},
			Attestation:      "none",
			PubKeyCredParams: supportedAlgorithms,
			RP: publicKeyCredentialRPOptions{
				ID:   rpID,
				Name: rpName,
			},
			User: publicKeyCredentialUserOptions{
				IDBase64:    userIDBase64,
				Name:        userName,
				DisplayName: userDisplayName,
			},
		},
	}

	if len(excludeKeyIDs) > 0 {
		var list []publicKeyCredentialExcludeCredentialsOptions
		for _, id := range excludeKeyIDs {
			list = append(list, publicKeyCredentialExcludeCredentialsOptions{
				Type:     "public-key",
				IDBase64: id,
				// keyID なんてそうそう重複しないでしょの気持ちで、transports は指定をサボることにする
			})
		}
		opt.PublicKey.ExcludeCredentials = list
	}

	return opt
}

// PublicCredentialAttestation をパースする (Assertionを検証するとき)
func ParseAttestationForVerification(jsonReader io.Reader, origin string) (*publicKeyCredentialAttestation, error) {
	bytes, err := io.ReadAll(jsonReader)
	if err != nil {
		return nil, err
	}

	var attestation publicKeyCredentialAttestation
	if err := json.Unmarshal(bytes, &attestation); err != nil {
		return nil, err
	}

	if attestation.ID == "" || attestation.RawID == "" {
		return nil, errors.New("attestationのIDが空だった")
	}

	if attestation.ID != attestation.RawID {
		return nil, errors.New("attestationのIDとrawIDが一致しなかった")
	}

	if !isSupportedAlgorithm(attestation.Response.PublicKeyAlgorithm) {
		return nil, fmt.Errorf("attestationのサポートしていない alg が渡された %d", attestation.Response.PublicKeyAlgorithm)
	}

	switch attestation.Response.PublicKeyAlgorithm {
	case algorithmRS256:
		_, err := parseRS256PublicKey(&attestation)
		if err != nil {
			return nil, fmt.Errorf("attestationの公開鍵のパースに失敗した %v", err)
		}
	default:
		// 直前で isSupportedAlgorithm を通しているので、明らかにおかしい
		panic(fmt.Sprintf("未知のアルゴリズム %d", attestation.Response.PublicKeyAlgorithm))
	}

	clientDataJSON, err := base64.RawStdEncoding.DecodeString(attestation.Response.ClientDataJSON)
	if err != nil {
		return nil, errors.Join(errors.New("attestationのclientDataJSONのパースに失敗した"), err)
	}

	var clientData authenticatorResponseClientData
	if err := json.Unmarshal(clientDataJSON, &clientData); err != nil {
		return nil, errors.Join(errors.New("attestationのclientDataのパースに失敗した"), err)
	}

	if clientData.Type != "webauthn.create" {
		return nil, fmt.Errorf("attestationのclientData.typeが未知 %s", clientData.Type)
	}

	if clientData.Origin != origin {
		return nil, fmt.Errorf("attestationのclientData.originが不正 %s", clientData.Origin)
	}

	if clientData.CrossOrigin {
		return nil, fmt.Errorf("attestationのclientData.crossOrigin=trueが不正")
	}

	attestation.Response.clientData = clientData

	return &attestation, nil
}

// PublicCredentialAttestation をパースする (登録時)
func ParseAttestationForRegistration(jsonReader io.Reader, challenge []byte, origin string) (*publicKeyCredentialAttestation, error) {
	attestation, err := ParseAttestationForVerification(jsonReader, origin)
	if err != nil {
		return nil, err
	}

	challengeBase64 := base64.RawURLEncoding.EncodeToString(challenge)
	if attestation.Response.clientData.Challenge != challengeBase64 {
		return nil, fmt.Errorf("attestationのchallengeが不正")
	}

	return attestation, nil
}
