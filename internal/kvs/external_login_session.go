package kvs

import (
	"context"
	"encoding/json"
)

type externalLoginSession struct {
	Nonce        string `json:"nonce"`
	State        string `json:"state"`
	Provider     string `json:"provider"`
	LoginSession string `json:"login_session"`
}

func ExternalLoginSession_set(nonce, state, provider, loginSession string) error {
	v, err := json.Marshal(externalLoginSession{
		Nonce:        nonce,
		State:        state,
		Provider:     provider,
		LoginSession: loginSession,
	})
	if err != nil {
		return err
	}

	key := "EXTERNAL_LOGIN_SESSION:" + state
	if err := Set(context.Background(), key, string(v), 120); err != nil {
		return err
	}

	return nil
}

func ExternalLoginSession_get(state string) (*externalLoginSession, error) {
	key := "EXTERNAL_LOGIN_SESSION:" + state
	v, err := Get(context.Background(), key)
	if err != nil {
		return nil, err
	}

	var s externalLoginSession
	if err := json.Unmarshal([]byte(v), &s); err != nil {
		return nil, err
	}

	return &s, nil
}

func ExternalLoginSession_delete(state string) {
	key := "EXTERNAL_LOGIN_SESSION:" + state
	Del(context.Background(), key)
}
