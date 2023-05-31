package main

import (
	"context"
	"database/sql"
	"fmt"
	"strings"

	"github.com/comame/accounts.comame.xyz/dashboard/core"
	"github.com/comame/accounts.comame.xyz/dashboard/db"
)

type relyingParty struct {
	ClientId     string   `json:"client_id"`
	RedirectUris []string `json:"redirect_uris"`
}

type listRelyingPartyResponse struct {
	Values []relyingParty `json:"values"`
}

func listRp(ctx context.Context) (*listRelyingPartyResponse, error) {
	db := db.DB

	rows, err := db.QueryContext(ctx, `
		SELECT P.client_id, U.redirect_uri
		FROM relying_parties P
		LEFT OUTER JOIN redirect_uris U ON P.client_id = U.client_id
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var rps = make(map[string]*relyingParty)

	for rows.Next() {
		var clientId string
		var redirectUriOpt sql.NullString
		if err := rows.Scan(&clientId, &redirectUriOpt); err != nil {
			return nil, err
		}

		_, ok := rps[clientId]
		if !ok {
			rps[clientId] = &relyingParty{
				ClientId:     clientId,
				RedirectUris: []string{},
			}
		}

		if redirectUriOpt.Valid {
			rps[clientId].RedirectUris = append(rps[clientId].RedirectUris, redirectUriOpt.String)
		}
	}

	res := listRelyingPartyResponse{
		Values: derefSlice(mapValues(rps)),
	}
	return &res, nil
}

type createRpResponse struct {
	RelyingParty relyingParty `json:"rp"`
	RawSecret    string       `json:"client_secret"`
}

func createRp(ctx context.Context, clientId string) (*createRpResponse, error) {
	secret, err := randomStr(32)
	if err != nil {
		return nil, err
	}

	hashed := core.CalculatePasswordHash(secret, clientId)

	db := db.DB

	if _, err := db.ExecContext(ctx, `
		INSERT INTO relying_parties
			(client_id, hashed_client_secret)
		VALUES
			(?, ?)
	`, clientId, hashed); err != nil {
		return nil, err
	}

	res := &createRpResponse{
		RelyingParty: relyingParty{
			ClientId:     clientId,
			RedirectUris: []string{},
		},
		RawSecret: secret,
	}

	return res, nil
}

func changeRpSecret(ctx context.Context, clientId string) (*createRpResponse, error) {
	secret, err := randomStr(32)
	if err != nil {
		return nil, err
	}

	hashed := core.CalculatePasswordHash(secret, clientId)

	db := db.DB

	if _, err := db.ExecContext(ctx, `
		UPDATE relying_parties
		SET hashed_client_secret=?
		WHERE client_id=?
	`, hashed, clientId); err != nil {
		return nil, err
	}

	res := &createRpResponse{
		RelyingParty: relyingParty{
			ClientId:     clientId,
			RedirectUris: []string{},
		},
		RawSecret: secret,
	}

	return res, nil
}

func deleteRp(ctx context.Context, clientId string) error {
	db := db.DB

	_, err := db.ExecContext(ctx, `
		DELETE from relying_parties
		WHERE client_id=?
	`, clientId)

	if err != nil {
		return err
	}

	return nil
}

func addRedirectUri(ctx context.Context, clientId string, redirectUri string) error {
	allowedPrefixes := []string{
		"https://",
		"http://localhost:808",
	}

	allowed := false
	for _, p := range allowedPrefixes {
		if strings.HasPrefix(redirectUri, p) {
			allowed = true
		}
	}
	if !allowed {
		return fmt.Errorf("redirect_uri prefix is not allowed")
	}

	db := db.DB
	if _, err := db.ExecContext(ctx, `
		INSERT INTO redirect_uris
			(client_id, redirect_uri)
		VALUES
			(?, ?)
	`, clientId, redirectUri); err != nil {
		return err
	}

	return nil
}

func removeRedirectUri(ctx context.Context, clientId string, redirectUri string) error {
	db := db.DB
	if _, err := db.ExecContext(ctx, `
		DELETE FROM redirect_uris
		WHERE client_id=? AND redirect_uri=?
	`, clientId, redirectUri); err != nil {
		return err
	}

	return nil
}
