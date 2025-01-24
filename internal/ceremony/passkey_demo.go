package ceremony

import (
	"encoding/json"
	"log"
	"net/http"
	"os"

	"github.com/comame/accounts.comame.xyz/internal/passkey"
)

func HandlePasskeyRegistrationOptionsDemo(w http.ResponseWriter) {
	userID := "test_user"

	challenge, err := passkey.CreateChallengeAndBindSession(w)
	if err != nil {
		log.Printf("パスキーのチャンレジ作成に失敗した %v", err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	excludeKeyIDs, err := passkey.ListBoundKeyIDs(userID)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	opt := passkey.CreateOptions(
		passkey.RelyingPartyID(os.Getenv("HOST")),
		"accounts.comame.xyz",
		userID,
		userID,
		userID,
		excludeKeyIDs,
		challenge,
	)

	j, err := json.Marshal(opt)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Add("Content-Type", "application/json")
	w.Write(j)
}

func HandleRegisterPasskeyDemo(w http.ResponseWriter, r *http.Request) {
	userID := "test_user"

	challenge, err := passkey.GetChallengeFromSession(w, r)
	if err != nil {
		log.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	attestation, err := passkey.ParseAttestationForRegistration(r.Body, challenge, os.Getenv("HOST"))
	if err != nil {
		log.Println("不正なAttestationを渡された", err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	if err := passkey.BindPublicKeyToUser(userID, *attestation); err != nil {
		log.Println("パスキーの紐づけに失敗", err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	{
		js, _ := json.MarshalIndent(attestation, "", "  ")
		log.Println(string(js))
	}

	w.Write([]byte{})
}

func HandlePasskeyGetOptionsDemo(w http.ResponseWriter) {
	challenge, err := passkey.CreateChallengeAndBindSession(w)
	if err != nil {
		log.Printf("パスキーのチャンレジ作成に失敗した %v", err)
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	opt := passkey.GetOptions(passkey.RelyingPartyID(os.Getenv("HOST")), challenge)

	j, err := json.Marshal(opt)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	w.Header().Add("Content-Type", "application/json")
	w.Write(j)
}

func HandlePasskeyVerify(w http.ResponseWriter, r *http.Request) {
	challenge, err := passkey.GetChallengeFromSession(w, r)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	assertion, err := passkey.ParseAssertion(r.Body, challenge, os.Getenv("HOST"))
	if err != nil {
		log.Println("assertionのパースに失敗", err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	unauthorizedUserID, err := passkey.AssumeUserID(assertion)
	if err != nil {
		log.Println("assertionからuserIDを取り出せなかった", err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	attestation, err := passkey.GetBoundPublicKey(unauthorizedUserID, *assertion)
	if err != nil {
		log.Println("対応するattestationがなかった")
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	userID, err := passkey.VerifyAssertion(*attestation, *assertion)
	if err != nil {
		log.Println("assertionの検証に失敗", err)
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	log.Println("成功！", userID)
	w.WriteHeader(http.StatusOK)
}
