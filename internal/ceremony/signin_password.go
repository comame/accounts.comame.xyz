package ceremony

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
)

// FIXME: 使っていないパラメーターを整理する
type signinPasswordAPIRequest struct {
	UserId         string `json:"user_id"`
	Password       string `json:"password"`
	RelyingPartyID string `json:"relying_party_id"`
	UserAgentID    string `json:"user_agent_id"`
	StateID        string `json:"state_id"`
}

func SigninWithPassword(w http.ResponseWriter, r io.Reader) {
	body, err := io.ReadAll(r)
	if err != nil {
		responseError(w, messageBadRequest)
		return
	}

	var request signinPasswordAPIRequest
	if err := json.Unmarshal(body, &request); err != nil {
		responseError(w, messageBadRequest)
		return
	}

	passwordIsOK, err := auth.AuthenticateByPassword(context.Background(), request.UserId, request.Password, request.RelyingPartyID, request.UserAgentID)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}
	if !passwordIsOK {
		responseError(w, messageInvalidCredential)
		return
	}

	roleIsOK, err := auth.Authorized(request.UserId, request.RelyingPartyID)
	if err != nil {
		responseError(w, messageBadRequest)
		return
	}
	if !roleIsOK {
		responseError(w, messageUnauthorized)
		return
	}

	authenticationResopnse, err := createAuthenticationResponse(request.UserId, request.StateID, request.RelyingPartyID)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	redirectURI, err := oidc.CreateRedirectURLFromAuthenticationResponse(authenticationResopnse)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	io.WriteString(w, fmt.Sprintf(`{ "location": "%s" }`, redirectURI))
}
