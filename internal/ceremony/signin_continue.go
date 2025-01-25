package ceremony

import (
	"log"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/cookie"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
)

func ContinueAuthenticated(w http.ResponseWriter, r *http.Request) {
	userID, err := getUserIDFromSigninSession(r)
	if err != nil {
		log.Println(err)
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	loginSessionCookie, err := r.Cookie(cookie.LoginSession)
	if err != nil {
		log.Println(err)
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	client_id := r.URL.Query().Get("client_id")
	if client_id == "" {
		log.Println(err)
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	authenticationResponse, err := createAuthenticationResponse(userID, loginSessionCookie.Value, client_id)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	redirectURI, err := oidc.CreateRedirectURLFromAuthenticationResponse(authenticationResponse)
	if err != nil {
		log.Println(err)
		responseError(w, messageBadRequest)
		return
	}

	w.Header().Set("Location", redirectURI)
	w.WriteHeader(http.StatusSeeOther)
}
