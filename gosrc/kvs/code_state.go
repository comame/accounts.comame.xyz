package kvs

import (
	"context"
	"encoding/json"
)

type codeState struct {
	Code        string `json:"code"`
	Aud         string `json:"aud"`
	Sub         string `json:"sub"`
	IDToken     string `json:"id_token"`
	Scope       string `json:"scope"`
	RedirectURI string `json:"redirect_uri"`
}

func CodeState_save(
	code, aud, sub, idToken, scope, redirectURI string,
) error {
	s := codeState{
		Code:        code,
		Aud:         aud,
		Sub:         sub,
		IDToken:     idToken,
		Scope:       scope,
		RedirectURI: redirectURI,
	}

	key := "CODE:" + code
	v, err := json.Marshal(s)
	if err != nil {
		return err
	}

	if err := Set(context.Background(), key, string(v), 5*60); err != nil {
		return err
	}

	return nil
}

func CodeState_get(code string) (*codeState, error) {
	key := "CODE:" + code
	v, err := Get(context.Background(), key)
	if err != nil {
		return nil, err
	}
	var r codeState
	if err := json.Unmarshal([]byte(v), &r); err != nil {
		return nil, err
	}
	return &r, nil
}
