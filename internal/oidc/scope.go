package oidc

import (
	"slices"
	"strings"
)

// scopes が target を満たすかどうかを返す
func ContainsScope(scopes, target string) bool {
	a := strings.Split(scopes, " ")
	b := strings.Split(target, " ")

	for _, v := range b {
		if !slices.Contains(a, v) {
			return false
		}
	}

	return true
}
