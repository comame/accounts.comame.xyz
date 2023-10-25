package oidc

import (
	"encoding/json"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/json-go/builder"
)

type userInfo struct {
	Sub               string `json:"sub"`
	Email             string `json:"email"`
	EmailVerified     bool   `json:"email_verified"`
	Name              string `json:"name"`
	PreferredUsername string `json:"preferred_username"`
	Profile           string `json:"profile"`
	Picture           string `json:"picture"`
}

func GetUserinfoJSON(token string) ([]byte, error) {
	sub, scope, err := FindAccessToken(token)
	if err != nil {
		return nil, err
	}

	uis, err := db.UserInfo_get(sub)
	if err != nil {
		return nil, err
	}

	var ui userInfo
	if err := json.Unmarshal([]byte(uis), &ui); err != nil {
		return nil, err
	}

	// omitempty を制御するのが大変なので、builder を使用する
	res := builder.Object(
		builder.Entry("sub", builder.String(sub)),
	)

	if hasScope(scope, "email") {
		res.MustSet("email", builder.String(ui.Email))
		res.MustSet("email_verified", builder.Bool(ui.EmailVerified))
	}

	if hasScope(scope, "profile") {
		res.MustSet("name", builder.String(ui.Name))
		res.MustSet("preferred_username", builder.String(ui.PreferredUsername))
		res.MustSet("profile", builder.String(ui.Profile))
		res.MustSet("picture", builder.String(ui.Picture))
	}

	j, err := res.Build()
	if err != nil {
		return nil, err
	}
	return j, nil
}
