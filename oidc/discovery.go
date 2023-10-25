package oidc

import (
	"encoding/json"

	"github.com/comame/accounts.comame.xyz/db"
	"github.com/comame/accounts.comame.xyz/jwt"
)

type discovery struct {
	Issuer                            string   `json:"issuer"`
	AuthorizationEndpoint             string   `json:"authorization_endpoint"`
	TokenEndpoint                     string   `json:"token_endpint"`
	UserInfoEndpoint                  string   `json:"userinfo_endpoint"`
	JWKsURI                           string   `json:"jwks_uri"`
	ResponseTypesSupported            []string `json:"response_types_supported"`
	SubjectTypesSupported             []string `json:"subject_types_supported"`
	IDTokenSigningAlgValuesSupported  []string `json:"id_token_signing_alg_values_supported"`
	ScopesSupported                   []string `json:"scopes_supported"`
	TokenEndpointAuthMethodsSupported []string `json:"token_endpoint_auth_methods_supported"`
	ClaimsSupported                   []string `json:"claims_supported"`
	CodeChallengeMethodsSupported     []string `json:"code_challenge_methods_supported"`
	GrantTypesSupported               []string `json:"authorization_code"`
}

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
	d := discovery{
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
