package passkey

import (
	"encoding/base64"
	"errors"
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

	str := base64.StdEncoding.EncodeToString(buf)

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

// CredentialCreationOptions を生成する
func CreateOptions(rpID, rpName string, userID []byte, userName, userDisplayName string, excludeKeyIDs []string, challenge []byte) credentialCreationOptions {
	userIDBase64 := base64.StdEncoding.EncodeToString(userID)
	challengeBase64 := base64.StdEncoding.EncodeToString(challenge)

	opt := credentialCreationOptions{
		PublicKey: credentialCreationPublicKeyOptions{
			ChallengeBase64: challengeBase64,
			AuthenticatorSelection: credentialCreationAuthenticatorSelectionOptions{
				AuthenticatorAttachment: "platform",
				RequireResidentKey:      true,
				ResidentKey:             "required",
				UserVerification:        "required",
			},
			PubKeyCredParams: []credentialCreationPubKeyCredParamsOptions{
				{
					// RS256
					Type: "public-key",
					Alg:  -257,
				},
				// {
				// 	// Ed25519
				// 	Type: "public-key",
				// 	Alg: -8,
				// },
				// {
				// 	// ES256
				// 	Type: "public-key",
				// 	Alg: -7,
				// },
			},
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
