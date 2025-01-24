package ceremony

import (
	"errors"
	"fmt"
	"io"
	"net/http"

	"github.com/comame/accounts.comame.xyz/internal/auth"
	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/jwt"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
	"github.com/comame/accounts.comame.xyz/internal/random"
	"github.com/comame/accounts.comame.xyz/internal/timenow"
)

var (
	messageBadRequest        = "bad_request"
	messageInvalidCredential = "invalid_credential"
	messageUnauthorized      = "unauthorized"
)

func responseError(w http.ResponseWriter, message string) {
	w.WriteHeader(http.StatusBadRequest)
	io.WriteString(w, fmt.Sprintf(`{ "error": "%s" }`, message))
}

// IDToken を発行する。sub が確定したのち呼ぶ。
func createAuthenticationResponse(sub, stateID, aud string) (*oidc.AuthenticationResponse, error) {
	authorized, err := auth.Authorized(sub, aud)
	if err != nil {
		return nil, err
	}
	if !authorized {
		return nil, errors.New("ロールがない")
	}

	state, err := kvs.LoginSession_get(stateID)
	if err != nil {
		return nil, err
	}
	kvs.LoginSession_delete(stateID)

	// TODO: userAgentID を使う？

	if state.RelyingPartyID != aud {
		return nil, errors.New("RelyingPartyID mismatch")
	}

	roles, err := db.Role_getUserRoles(sub)
	if err != nil {
		return nil, err
	}

	now := timenow.Now().Unix()

	// TODO: auth_at に対応する
	// TODO: email, email_verified, name, preferred_username, profile, picture に対応する
	claim := jwt.Payload{
		Iss:   "https://accounts.comame.xyz",
		Sub:   sub,
		Aud:   state.RelyingPartyID,
		Iat:   now,
		Exp:   now + int64(5*60),
		Roles: roles,
	}
	if state.Nonce != "" {
		claim.Nonce = state.Nonce
	}

	kp, err := db.RSAKeypair_get()
	if err != nil {
		return nil, err
	}
	priv, err := jwt.DecodeRSAPrivKeyPem(kp.Private)
	if err != nil {
		return nil, err
	}
	token, err := jwt.EncodeJWT(jwt.Header{
		Alg: "RS256",
		Typ: "JWT",
		Kid: kp.Kid,
	}, claim, priv)
	if err != nil {
		return nil, err
	}

	switch oidc.Flow(state.Flow) {
	case oidc.FlowCode:
		code, err := random.String(32)
		if err != nil {
			return nil, err
		}
		if err := kvs.CodeState_save(code, aud, sub, token, state.Scopes, state.RedirectURI); err != nil {
			return nil, err
		}
		res := &oidc.AuthenticationResponse{
			Code:        code,
			Flow:        oidc.FlowCode,
			RedirectURI: state.RedirectURI,
		}
		if state.State != "" {
			res.State = state.State
		}
		return res, nil
	case oidc.FlowImplicit:
		res := &oidc.AuthenticationResponse{
			IDToken:     token,
			Flow:        oidc.FlowImplicit,
			RedirectURI: state.RedirectURI,
		}
		if state.State != "" {
			res.State = state.State
		}
		return res, nil
	default:
		return nil, errors.New("invalid state")
	}
}
