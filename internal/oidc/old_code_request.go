package oidc

import (
	"database/sql"
	"errors"
	"net/url"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
)

func ParseCodeRequest(values url.Values) codeRequest {
	return codeRequest{
		ClientID:     values.Get("client_id"),
		ClientSecret: values.Get("client_secret"),
		GrantType:    values.Get("grant_type"),
		Code:         values.Get("code"),
		RedirectURI:  values.Get("redirect_uri"),
	}
}

func HandleCodeRequest(r codeRequest) (*codeResponse, error) {
	rp, err := db.RelyingParty_select(r.ClientID)
	if err != nil && errors.Is(err, sql.ErrNoRows) {
		return &codeResponse{
			Error: "invalid_client",
		}, nil
	}
	if err != nil {
		return nil, err
	}

	if auth.CalculatePasswordHash(r.ClientSecret, r.ClientID) != rp.HashedClientSecret {
		return &codeResponse{
			Error: "invalid_client",
		}, nil
	}

	state, err := kvs.CodeState_get(r.Code)
	if err != nil {
		return &codeResponse{
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

	at, exp, err := CreateAccessToken(state.Sub, state.Scope)
	if err != nil {
		return nil, err
	}

	return &codeResponse{
		AccessToken: at,
		TokenType:   "Bearer",
		ExpiresIn:   exp,
		Scope:       state.Scope,
		IDToken:     state.IDToken,
	}, nil
}
