package main

import (
	"context"
	"crypto/rand"
	"fmt"

	"github.com/comame/accounts.comame.xyz/dashboard/kvs"
)

const accountOrigin = "https://accounts.comame.xyz"

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
