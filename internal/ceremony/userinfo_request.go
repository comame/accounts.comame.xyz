package ceremony

import (
	"database/sql"
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"strings"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
	"github.com/comame/json-go/builder"
)

func HandleUserInfoRequest(w http.ResponseWriter, r *http.Request) {
	var accessToken string

	// RFC6750 2.1
	authHeader := r.Header.Get("Authorization")
	if authHeader != "" {
		if len(authHeader) < len("Bearer a") {
			w.Header().Set("WWW-Authenticate", `Bearer error="invalid_request"`)
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		if !strings.HasPrefix(authHeader, "Bearer ") {
			w.Header().Set("WWW-Authenticate", `Bearer error="invalid_request"`)
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		accessToken = strings.TrimPrefix(authHeader, "Bearer ")
	}

	// RFC6750 2.3
	q := r.URL.Query().Get("access_token")
	if q != "" && accessToken != "" {
		w.Header().Set("WWW-Authenticate", `Bearer error="invalid_request"`)
		w.WriteHeader(http.StatusUnauthorized)
		return
	}
	if q != "" {
		accessToken = q
	}

	// RFC6750 2.1
	if accessToken == "" {
		if err := r.ParseForm(); err != nil {
			w.Header().Set("WWW-Authenticate", `Bearer error="invalid_request"`)
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
		accessToken = r.Form.Get("access_token")
		if accessToken == "" {
			w.Header().Set("WWW-Authenticate", `Bearer error="invalid_request"`)
			w.WriteHeader(http.StatusUnauthorized)
			return
		}
	}

	j, err := getUserinfoJSON(accessToken)
	if err != nil {
		log.Println(err)
		w.WriteHeader(http.StatusUnauthorized)
		w.Header().Set("WWW-Authenticate", `Bearer error="invalid_token"`)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.Write(j)
}

func getUserinfoJSON(token string) ([]byte, error) {
	sub, scope, err := findAccessToken(token)
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

	var ui oidc.UserInfoResponse
	if err := json.Unmarshal([]byte(uis), &ui); err != nil {
		return nil, err
	}

	// omitempty を制御するのが大変なので、builder を使用する
	res := builder.Object(
		builder.Entry("sub", builder.String(sub)),
	)

	if oidc.ContainsScope(scope, "email") {
		res.MustSet("email", builder.String(ui.Email))
		res.MustSet("email_verified", builder.Bool(ui.EmailVerified))
	}

	if oidc.ContainsScope(scope, "profile") {
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
