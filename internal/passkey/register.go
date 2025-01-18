package passkey

import (
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

// .publicKey.rp.id に入る値 (おおむねホスト名) を返す
func RelyingPartyID() string {
	return "localhost"
}

// challenge を生成してセッションに紐づける
func CreateChallengeAndBindSession(userID string, w http.ResponseWriter) ([]byte, error) {
	buf := make([]byte, 32)
	n, err := random.Bytes(buf)
	if err != nil {
		return nil, err
	}
	if n != 32 {
		return nil, errors.New("challengeが32バイトではない")
	}

	str := base64.RawURLEncoding.EncodeToString(buf)

	if err := kvs.PasskeyChallenge_save(str, userID); err != nil {
		return nil, err
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "pkc",
		Value:    str,
		MaxAge:   10 * 60,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
	})

	return buf, nil
}

// セッションに紐づけられたchallengeを取得する
func GetChallengeFromSession(userID string, r *http.Request) ([]byte, error) {
	cookie, err := r.Cookie("pkc")
	if err != nil {
		return nil, err
	}

	str := cookie.Value
	buf, err := base64.RawURLEncoding.DecodeString(str)
	if err != nil {
		return nil, err
	}

	defer kvs.PasskeyChallenge_delete(str)
	kvsUserID, err := kvs.PasskeyChallenge_get(str)
	if err != nil {
		return nil, err
	}
	if kvsUserID != userID {
		return nil, errors.New("パスキー登録時にkvsに保存されたuserIDがセッションと異なる")
	}

	return buf, nil
}

// CredentialCreationOptions を生成する
func CreateOptions(rpID, rpName string, userID []byte, userName, userDisplayName string, excludeKeyIDs []string, challenge []byte) credentialCreationOptions {
	userIDBase64 := base64.RawURLEncoding.EncodeToString(userID)
	challengeBase64 := base64.RawURLEncoding.EncodeToString(challenge)

	opt := credentialCreationOptions{
		PublicKey: credentialCreationPublicKeyOptions{
			ChallengeBase64: challengeBase64,
			AuthenticatorSelection: credentialCreationAuthenticatorSelectionOptions{
				AuthenticatorAttachment: "platform",
				RequireResidentKey:      true,
				ResidentKey:             "required",
				UserVerification:        "required",
			},
			PubKeyCredParams: supportedAlgorithms,
			RP: credentialCreationRPOptions{
				ID:   rpID,
				Name: rpName,
			},
			User: credentialCreationUserOptions{
				IDBase64:    userIDBase64,
				Name:        userName,
				DisplayName: userDisplayName,
			},
		},
	}

	if len(excludeKeyIDs) > 0 {
		var list []credentialCreationExcludeCredentialsOptions
		for _, id := range excludeKeyIDs {
			list = append(list, credentialCreationExcludeCredentialsOptions{
				Type:     "public-key",
				IDBase64: id,
			})
		}
		opt.PublicKey.ExcludeCredentials = list
	}

	return opt
}

// PublicCredentialAttestation をパースする (Assertionを検証するとき)
func ParseAttestationForVerification(jsonReader io.Reader, origin string) (*PublicCredentialAttestation, error) {
	bytes, err := io.ReadAll(jsonReader)
	if err != nil {
		return nil, err
	}

	var attestation PublicCredentialAttestation
	if err := json.Unmarshal(bytes, &attestation); err != nil {
		return nil, err
	}

	if attestation.ID == "" || attestation.RawID == "" {
		return nil, errors.New("attestationのIDが空だった")
	}

	if attestation.ID != attestation.RawID {
		return nil, errors.New("attestationのIDとrawIDが一致しなかった")
	}

	alg, ok := getAlgFromNumber(attestation.Response.PublicKeyAlgorithm)
	if !ok {
		return nil, fmt.Errorf("attestationのサポートしていない alg が渡された %d", attestation.Response.PublicKeyAlgorithm)
	}

	switch alg {
	case "RS256":
		_, err := parseRS256(attestation.Response.PublicKey)
		if err != nil {
			return nil, fmt.Errorf("attestationの公開鍵のパースに失敗した %v", err)
		}
	default:
		panic("未知のアルゴリズム名 " + alg)
	}

	clientDataJSON, err := base64.RawStdEncoding.DecodeString(attestation.Response.ClientDataJSON)
	if err != nil {
		return nil, errors.Join(errors.New("attestationのclientDataJSONのパースに失敗した"), err)
	}

	var clientData authenticatorAttestationResponseClientData
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
func ParseAttestationForRegistration(jsonReader io.Reader, challenge []byte, origin string) (*PublicCredentialAttestation, error) {
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
