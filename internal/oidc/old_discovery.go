package oidc

import (
	"encoding/json"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/jwt"
)

func GetDiscoveryCertsJSON() ([]byte, error) {
	keypair, err := db.RSAKeypair_get()
	if err != nil {
		return nil, err
	}

	k, err := jwt.DecodeRSAPubKeyPem(keypair.Public)
	if err != nil {
		return nil, err
	}

	key, err := jwt.EncodeToJWK(k, keypair.Kid)
	if err != nil {
		return nil, err
	}

	jwk := jwt.JWK{
		Keys: []jwt.JWKKey{*key},
	}

	js, err := json.MarshalIndent(jwk, "", "    ")
	if err != nil {
		return nil, err
	}

	return js, nil
}

func GetDiscoveryConfigurationJSON(issuer string) ([]byte, error) {
	d := Discovery{
		Issuer:                            issuer,
		AuthorizationEndpoint:             issuer + "/authenticate",
		TokenEndpoint:                     issuer + "/code",
		UserInfoEndpoint:                  issuer + "/userinfo",
		JWKsURI:                           issuer + "/certs",
		ResponseTypesSupported:            []string{"code", "id_token"},
		SubjectTypesSupported:             []string{"public"},
		IDTokenSigningAlgValuesSupported:  []string{"RS256"},
		ScopesSupported:                   []string{"openid", "email", "profile"},
		TokenEndpointAuthMethodsSupported: []string{"client_secret_basic", "client_secret_post"},
		ClaimsSupported:                   []string{"aud", "exp", "iat", "iss", "sub"},
		CodeChallengeMethodsSupported:     []string{},
		GrantTypesSupported:               []string{"authorization_code"},
	}
	j, err := json.MarshalIndent(d, "", "    ")
	if err != nil {
		return nil, err
	}
	return j, nil
}
