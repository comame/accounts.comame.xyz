package main

import (
	"context"

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
		var redirectUri string
		if err := rows.Scan(&clientId, &redirectUri); err != nil {
			return nil, err
		}

		_, ok := rps[clientId]
		if ok {
			rps[clientId].RedirectUris = append(rps[clientId].RedirectUris, redirectUri)
		} else {
			rps[clientId] = &relyingParty{
				ClientId:     clientId,
				RedirectUris: []string{redirectUri},
			}
		}
	}

	res := listRelyingPartyResponse{
		Values: derefSlice(mapValues(rps)),
	}
	return &res, nil
}
