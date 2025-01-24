package ceremony

import (
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
)

func HandleCodeRequest(w http.ResponseWriter, r *http.Request) {
	w.Header().Add("Content-Type", "application/json;charset=UTF-8")
	w.Header().Add("Cache-Control", "no-store")
	w.Header().Add("Pragma", "no-cache")

	if err := r.ParseForm(); err != nil {
		responseError(w, messageBadRequest)
		return
	}
	req := oidc.CodeRequest{
		ClientID:     r.Form.Get("client_id"),
		ClientSecret: r.Form.Get("client_secret"),
		GrantType:    r.Form.Get("grant_type"),
		Code:         r.Form.Get("code"),
		RedirectURI:  r.Form.Get("redirect_uri"),
	}

	// client_secret_basic
	clientID, clientSecret, ok := r.BasicAuth()
	if ok && (req.ClientID != "" || req.ClientSecret != "") {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "invalid_request" }`)
		return
	}
	if ok {
		req.ClientID = clientID
		req.ClientSecret = clientSecret
	}

	res, err := handleCodeRequestInternal(req)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "invalid_request" }`)
		return
	}

	j, err := json.Marshal(res)
	if err != nil {
		fmt.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "invalid_request" }`)
		return
	}

	if res.Error != "" {
		w.WriteHeader(http.StatusBadRequest)
		w.Write(j)
		return
	}

	w.Write(j)
}

func handleCodeRequestInternal(r oidc.CodeRequest) (*oidc.CodeResponse, error) {
	rp, err := db.RelyingParty_select(r.ClientID)
	if err != nil && errors.Is(err, sql.ErrNoRows) {
		return &oidc.CodeResponse{
			Error: "invalid_client",
		}, nil
	}
	if err != nil {
		return nil, err
	}

	if auth.CalculatePasswordHash(r.ClientSecret, r.ClientID) != rp.HashedClientSecret {
		return &oidc.CodeResponse{
			Error: "invalid_client",
		}, nil
	}

	state, err := kvs.CodeState_get(r.Code)
	if err != nil {
		return &oidc.CodeResponse{
			Error: "invalid_grant",
		}, nil
	}

	kvs.CodeState_delete(r.Code)

	if r.ClientID != state.Aud {
		return nil, errors.New("client_id が保存されたものと違う")
	}

	if r.RedirectURI != state.RedirectURI {
		return nil, errors.New("redirect_uri が保存されたものと違う")
	}

	if r.GrantType != "authorization_code" {
		return nil, errors.New("grant_type が authorization_code ではない")
	}

	at, exp, err := createAccessToken(state.Sub, state.Scope)
	if err != nil {
		return nil, err
	}

	return &oidc.CodeResponse{
		AccessToken: at,
		TokenType:   "Bearer",
		ExpiresIn:   exp,
		Scope:       state.Scope,
		IDToken:     state.IDToken,
	}, nil
}
