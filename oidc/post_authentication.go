package oidc

import (
	"errors"

	"github.com/comame/accounts.comame.xyz/auth"
	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/jwt"
	"github.com/comame/accounts.comame.xyz/kvs"
	"github.com/comame/accounts.comame.xyz/random"
	"github.com/comame/accounts.comame.xyz/timenow"
)

// IDToken を発行する。sub が確定したのち呼ぶ。
func PostAuthentication(
	sub, stateID, aud, userAgentID string,
	loginType auth.AuthenticationMethod,
) (*AuthenticationResponse, error) {
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

	flow := Flow(state.Flow)
	switch flow {
	case FlowCode:
		code, err := random.String(32)
		if err != nil {
			return nil, err
		}
		if err := kvs.CodeState_save(code, aud, sub, token, state.Scopes, state.RedirectURI); err != nil {
			return nil, err
		}
		res := &AuthenticationResponse{
			Code:        code,
			Flow:        FlowCode,
			RedirectURI: state.RedirectURI,
		}
		if state.State != "" {
			res.State = state.State
		}
		return res, nil
	case FlowImplicit:
		res := &AuthenticationResponse{
			IDToken:     token,
			Flow:        FlowImplicit,
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
