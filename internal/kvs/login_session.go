package kvs

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/comame/accounts.comame.xyz/internal/oidc"
	"github.com/comame/accounts.comame.xyz/internal/random"
)

type loginSession struct {
	ID             string    `json:"id"`
	RelyingPartyID string    `json:"relying_party_id"`
	Flow           oidc.Flow `json:"flow"`
	RedirectURI    string    `json:"redirect_uri"`
	Scopes         string    `json:"scopes"`
	State          string    `json:"state"`
	Nonce          string    `json:"nonce"`
}

func LoginSession_save(
	sub, redirectURI, scope, state, nonce string, flow oidc.Flow,
) (string, error) {
	id, err := random.String(64)
	if err != nil {
		return "", err
	}

	s := loginSession{
		ID:             id,
		RelyingPartyID: sub,
		Flow:           flow,
		RedirectURI:    redirectURI,
		Scopes:         scope,
		State:          state,
		Nonce:          nonce,
	}
	b, err := json.Marshal(s)
	if err != nil {
		return "", err
	}

	key := fmt.Sprintf("%s:%s", "LOGINSESSION", id)
	if err := Set(context.Background(), key, string(b), 5*60); err != nil {
		return "", err
	}

	return id, nil
}

func LoginSession_get(id string) (*loginSession, error) {
	key := fmt.Sprintf("%s:%s", "LOGINSESSION", id)
	s, err := Get(context.Background(), key)
	if err != nil {
		return nil, err
	}

	var v loginSession
	if err := json.Unmarshal([]byte(s), &v); err != nil {
		return nil, err
	}

	return &v, nil
}

func LoginSession_delete(id string) {
	key := fmt.Sprintf("%s:%s", "LOGINSESSION", id)
	Del(context.Background(), key)
}
