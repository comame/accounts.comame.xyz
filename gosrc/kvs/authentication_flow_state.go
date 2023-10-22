package kvs

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/comame/accounts.comame.xyz/random"
)

type authenticationFlowState struct {
	ID             string `json:"id"`
	RelyingPartyID string `json:"relying_party_id"`
	Flow           int    `json:"flow"`
	RedirectURI    string `json:"redirect_uri"`
	Scopes         string `json:"scopes"`
	State          string `json:"state"`
	Nonce          string `json:"nonce"`
}

func AuthenticationFlowState_save(
	sub, redirectURI, scope, state, nonce string, flow int,
) (string, error) {
	id, err := random.String(64)
	if err != nil {
		return "", err
	}

	s := authenticationFlowState{
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

	key := fmt.Sprintf("%s:%s", "AUTH_FLOW_STATE", id)
	if err := Set(context.Background(), key, string(b), 5*60); err != nil {
		return "", err
	}

	return id, nil
}

func AuthenticationFlowState_get(id string) (*authenticationFlowState, error) {
	key := fmt.Sprintf("%s:%s", "AUTH_FLOW_STATE", id)
	s, err := Get(context.Background(), key)
	if err != nil {
		return nil, err
	}

	var v authenticationFlowState
	if err := json.Unmarshal([]byte(s), &v); err != nil {
		return nil, err
	}

	return &v, nil
}

func AuthenticationFlowState_delete(id string) {
	key := fmt.Sprintf("%s:%s", "AUTH_FLOW_STATE", id)
	Del(context.Background(), key)
}
