package oidc

import (
	"strings"

	"golang.org/x/exp/slices"
)

// scopes が target を満たすかどうかを返す
func scopeWithin(scopes, target string) bool {
	a := strings.Split(scopes, " ")
	b := strings.Split(target, " ")

	for _, v := range b {
		if !slices.Contains(a, v) {
			return false
		}
	}

	return true
}
