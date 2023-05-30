package main

import (
	"context"
	"crypto/rand"
	"crypto/rsa"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"math/big"
	"net/http"
	"net/url"
	"strings"

	"github.com/comame/accounts.comame.xyz/dashboard/kvs"
	"github.com/golang-jwt/jwt/v4"
)

// const accountOrigin = "https://accounts.comame.xyz"
const accountOrigin = "http://localhost:8080"

func randomStr(length uint) (string, error) {
	const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"

	bytes := make([]byte, length)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}

	var str string
	for _, v := range bytes {
		str += string(letters[int(v)%len(letters)])
	}

	return str, nil
}

func createAuthUrl(ctx context.Context) (string, error) {
	state, err := randomStr(16)
	if err != nil {
		return "", nil
	}

	nonce, err := randomStr(16)
	if err != nil {
		return "", nil
	}

	origin := env.Host

	key := fmt.Sprintf("nonce:%s:%s", state, nonce)
	if err := kvs.Set(ctx, key, "_", 60); err != nil {
		return "", err
	}

	redirectUri := origin + "/dash/callback"

	authUrl := fmt.Sprintf("%s/authenticate?client_id=accounts.comame.xyz&redirect_uri=%s&scope=openid&response_type=code&state=%s&nonce=%s&prompt=login", accountOrigin, redirectUri, state, nonce)
	return authUrl, nil
}

func callbackAndIssueToken(ctx context.Context, state, code string) (string, error) {
	origin := env.Host
	redirectUri := origin + "/dash/callback"

	clientId := "accounts.comame.xyz"

	clientSecret := env.ClientSecret

	codeRequest := url.Values{
		"grant_type":    {"authorization_code"},
		"code":          {code},
		"redirect_uri":  {redirectUri},
		"client_id":     {clientId},
		"client_secret": {clientSecret},
	}

	res, err := http.Post(
		accountOrigin+"/code",
		"application/x-www-form-urlencoded",
		strings.NewReader(codeRequest.Encode()),
	)
	if err != nil {
		return "", logErr(err)
	}

	bytes, err := io.ReadAll(res.Body)
	if err != nil {
		return "", logErr(err)
	}

	type codeResponse_t struct {
		IdToken string `json:"id_token"`
	}
	var codeResponse codeResponse_t
	if err := json.Unmarshal(bytes, &codeResponse); err != nil {
		return "", logErr(err)
	}

	idToken := codeResponse.IdToken

	jwtToken, err := jwt.Parse(idToken, func(t *jwt.Token) (interface{}, error) {
		if _, ok := t.Method.(*jwt.SigningMethodRSA); !ok {
			return "", fmt.Errorf("invalid JWT signing method")
		}

		pubkey, err := getJwkPublicKey()
		if err != nil {
			return nil, logErr(err)
		}

		return pubkey, nil
	})
	if err != nil {
		return "", logErr(err)
	}

	claims, ok := jwtToken.Claims.(jwt.MapClaims)
	if !ok {
		return "", fmt.Errorf("invalid JWT claims")
	}

	if !jwtToken.Valid {
		return "", fmt.Errorf("invalid JWT claims")
	}

	if !claims.VerifyIssuer(accountOrigin, true) {
		return "", fmt.Errorf("invalid JWT iss field")
	}

	if !claims.VerifyAudience("accounts.comame.xyz", true) {
		return "", fmt.Errorf("invalid JWT aud field")
	}

	nonce, ok := claims["nonce"]
	if !ok {
		return "", fmt.Errorf("nonce is not present in JWT")
	}

	subject, ok := claims["sub"]
	if !ok {
		return "", fmt.Errorf("sub is not present in JWT")
	}

	if subject != "admin" {
		return "", fmt.Errorf("sub is not admin")
	}

	key := fmt.Sprintf("nonce:%s:%s", state, nonce)
	v, err := kvs.Get(ctx, key)
	if err != nil {
		return "", fmt.Errorf("invalid state or nonce")
	}
	if v != "_" {
		return "", fmt.Errorf("invalid state or nonce")
	}

	kvs.Del(ctx, key)

	token, err := randomStr(16)
	if err != nil {
		return "", err
	}

	key = fmt.Sprintf("token:%s", token)
	if err := kvs.Set(ctx, key, "_", 5*60); err != nil {
		return "", err
	}

	return token, nil
}

func getJwkPublicKey() (*rsa.PublicKey, error) {
	type jwkKey_t struct {
		N   string `json:"n"`
		Kty string `json:"kty"`
		Alg string `json:"alg"`
		E   string `json:"e"`
		Use string `json:"use"`
	}

	type jwk_t struct {
		Keys []jwkKey_t `json:"keys"`
	}

	res, err := http.Get(accountOrigin + "/certs")
	if err != nil {
		return nil, logErr(err)
	}
	bytes, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, logErr(err)
	}

	var jwk jwk_t
	if err := json.Unmarshal(bytes, &jwk); err != nil {
		return nil, logErr(err)
	}

	var key *jwkKey_t = nil
	for _, k := range jwk.Keys {
		if k.Kty == "RSA" && k.Alg == "RS256" && k.Use == "sig" {
			key = &k
		}
	}

	if key == nil {
		return nil, fmt.Errorf("valid jwk not found")
	}

	nBytes, err := base64.RawURLEncoding.DecodeString(key.N)
	if err != nil {
		return nil, fmt.Errorf("invalid form jwk.n")
	}

	eBytes, err := base64.RawURLEncoding.DecodeString(key.E)
	if err != nil {
		return nil, fmt.Errorf("invalid form jwk.e")
	}

	n := new(big.Int).SetBytes(nBytes)
	e := int(new(big.Int).SetBytes(eBytes).Int64())

	pubkey := &rsa.PublicKey{
		E: e,
		N: n,
	}

	return pubkey, nil
}

func authorized(ctx context.Context, token string) bool {
	key := fmt.Sprintf("token:%s", token)
	_, err := kvs.Get(ctx, key)
	return err == nil
}
