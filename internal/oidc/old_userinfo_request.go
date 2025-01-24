package oidc

import (
	"database/sql"
	"encoding/json"
	"errors"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/json-go/builder"
)

func GetUserinfoJSON(token string) ([]byte, error) {
	sub, scope, err := FindAccessToken(token)
	if err != nil {
		return nil, err
	}

	uis, err := db.UserInfo_get(sub)
	if err != nil && errors.Is(err, sql.ErrNoRows) {
		res := builder.Object(
			builder.Entry("sub", builder.String(sub)),
		)
		return res.MustBuild(), nil
	}
	if err != nil {
		return nil, err
	}

	var ui UserInfoResponse
	if err := json.Unmarshal([]byte(uis), &ui); err != nil {
		return nil, err
	}

	// omitempty を制御するのが大変なので、builder を使用する
	res := builder.Object(
		builder.Entry("sub", builder.String(sub)),
	)

	if ContainsScope(scope, "email") {
		res.MustSet("email", builder.String(ui.Email))
		res.MustSet("email_verified", builder.Bool(ui.EmailVerified))
	}

	if ContainsScope(scope, "profile") {
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
