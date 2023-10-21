package jwt_test

import (
	"testing"

	"github.com/comame/accounts.comame.xyz/jwt"
)

const privPem = `-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEApv4op2E1U8Gff9aip3PHsfvBYlI/mNlOiz7PMy40znRs6NEw
e6uFyhPgxqx3uF6KHv1k2md9v43LSmb2pZdOZP5LLgqAHLgJHhYW3LPYS1RV5V5G
DQyjQFaHHqEAq+M9AR6RfqbCRJcjMlSrcfdLcRAFaAKOrYX8mM1fiDF3qxs+Ga2i
JI4dDiRzDSpxVItCT8FG2UO+xXZJmBiX9Hw38M6bPSZv9hZcc+kzr4ysQnYeK3z8
vOsteHd/3BfJuhkfZ4Jn8wLRAUoBk3DRzkxtLXDpuIgYOLXEAy+/0sJBhw3TCY3J
Z6j15SqPyQnraBUfBGzWHr/KaR90MJ/BgPaHDwIDAQABAoIBAQCaV3YDnZjIHMDq
StpyolQDcIg/83zMLKynjhm7WA1+c6TlWdLRuqohJ1YsyeKtCNPn6JN6OQTEMq2d
yqRUUBAoz890ckgOQxpKMGhY+/ABT6VEJWnhTbjlEhQ4ft47//62NiaLRF1xP69M
KW18G/AiQ4h7pUFxp9Da+ZY+LlpITZslb4nozf7/sEQOTon15cUN+kzbT6f/4Xkl
EkvV7D83azRKO1M2gX6/k/deVN59C8yBS6IhK8ilpUeDa04wISWvtTdx+jAxTtrg
TB+6SghAoxO+cXui3E8URH5gIbuKN4LKxTmTeEm7RavdBUNFFKVJ+0f1tKA9rzbT
6Q8Kn7DhAoGBANWPvycSlt2NONrkPyM+jvxG+BQDIEj+i9lyvXDy7BKCjQYXHs09
7oK+CPUBqGPeXTqBn5ebB8SfDOWi90FaIEvk75P72zWxq79I/oGQ9QXB3nzf1Ws4
ktZwl1S2Uvmx5mbM97k6mjsA2HbgXjphn39YGYrJgSaLkYhCcGg4p4iDAoGBAMgt
YSdNTfULbh9xlb7CWuXpJD1YSrNdn0FF6x2juGS1mqRbgOjaQEYJT16AqyODE0l7
Ed9FbrhZjU31oN9bKB7AuQm6dQH/RTeGjMDnkodX6ejXhrrG7a80r+fno2smAAda
FhH3DbGYgIYBbrZOhylfzwR2AUx4nSsPKwZ/YAmFAoGACkcaufSpEgyD2fT2HOob
04RO2Be2bAzkChj9iPwH5HQn+U0PjG0Yl24x3CiyQ+wlrHUkixVI8Gt/IWYQZDLE
LyLbbNHIIPNmApSuuumRAS/tVzEmgjx8xJkyjz8fCylGn1fp7B4n4gMOZ9owbrrY
BDbnM8iy0HoTDO21ny5q0aUCgYAEw8/EN7rbUmJUrUd7OvUe8+XA8BEXI+teimOG
WRdSjmAX+XWgFVHiWOZiwX1RYxVacmuCfQydpsQOTwJ9TpYt5TiCWsXePk1PfQxs
qxZ50kXnHPpAB+wwd3iYdJMXQdhOVH6h/td8mry0c+RGwqfE/FrZKFXvlA9prvq/
NNj8YQKBgCMgHvcggzbH8i2+YhBjpKx4b/eNgqp5mQebwrS0DhdFh7RK5RghG+y+
NqLQN7xfKbnJWMWpL/WkVpY3scZDeWPdny4SQblCSXeMUJOOT7ognNNPcdqY5WLJ
YwVHzxY6R7QS/MunVKMNqSi7PdCvd5Y5ScciNyImtCjUtjuEdgAu
-----END RSA PRIVATE KEY-----`

const pubPem = `-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApv4op2E1U8Gff9aip3PH
sfvBYlI/mNlOiz7PMy40znRs6NEwe6uFyhPgxqx3uF6KHv1k2md9v43LSmb2pZdO
ZP5LLgqAHLgJHhYW3LPYS1RV5V5GDQyjQFaHHqEAq+M9AR6RfqbCRJcjMlSrcfdL
cRAFaAKOrYX8mM1fiDF3qxs+Ga2iJI4dDiRzDSpxVItCT8FG2UO+xXZJmBiX9Hw3
8M6bPSZv9hZcc+kzr4ysQnYeK3z8vOsteHd/3BfJuhkfZ4Jn8wLRAUoBk3DRzkxt
LXDpuIgYOLXEAy+/0sJBhw3TCY3JZ6j15SqPyQnraBUfBGzWHr/KaR90MJ/BgPaH
DwIDAQAB
-----END PUBLIC KEY-----`

const idToken = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJpc3N1ZXIiLCJzdWIiOiJzdWJzY3JpYmVyIiwiYXVkIjoiYXVkaWVuY2UiLCJleHAiOjAsImlhdCI6MCwibm9uY2UiOiJub25jZSIsInJvbGVzIjpudWxsfQ.eDbxo-nwfDV6ol1F7k2nAyHhkeEHBO15wRYw7gsAcZipxvaMnLHRrF-sIursgGUiYw9VEA4IX8HqvmVPxrS9tduH97qjDA9iC0EGZaQri9kalW9mA_pa3jfPo4kilUYutSVf3CHppHjQUaA1980VjUkiyWLFCwtPzZitbO_maPr9oZi6oetYKUIqswgDgXZcmW7DcVJB19mlvw44VOsTvSca9Kt7TZseGJnga1savydX-rwEKVp5Sejfi2euYp9oCvJs3oZfTrTG_Qd2L2DnbVd3efDTUePcPe0CcMiG54NybGf6w-bvChbwn-4SqPLFu7IMBULTJDM2c1JbKho-_w"

func TestEncode(t *testing.T) {
	key, err := jwt.DecodeRSAPrivKeyPem(privPem)
	if err != nil {
		t.Fatal(err)
	}

	got, err := jwt.EncodeJWT(jwt.Header{
		Typ: "JWT",
		Alg: "RS256",
	}, jwt.Payload{
		Iss:   "issuer",
		Sub:   "subscriber",
		Aud:   "audience",
		Exp:   0,
		Iat:   0,
		Nonce: "nonce",
	}, key)
	if err != nil {
		t.Fatal(err)
	}

	if got != idToken {
		t.Fatal("invalid idtoken")
	}
}

func TestDecode(t *testing.T) {
	key, _ := jwt.DecodeRSAPubKeyPem(pubPem)

	payload, err := jwt.DecodeJWT(idToken, key)
	if err != nil {
		t.Fatal(err)
	}

	if payload.Iss != "issuer" ||
		payload.Sub != "subscriber" ||
		payload.Aud != "audience" ||
		payload.Exp != 0 ||
		payload.Iat != 0 ||
		payload.Nonce != "nonce" {
		t.Fatal("wrong payload")
	}
}

func TestEncodeToJWK(t *testing.T) {
	key, _ := jwt.DecodeRSAPubKeyPem(pubPem)

	k, _ := jwt.EncodeToJWK(key, "1")

	if k.N != "pv4op2E1U8Gff9aip3PHsfvBYlI_mNlOiz7PMy40znRs6NEwe6uFyhPgxqx3uF6KHv1k2md9v43LSmb2pZdOZP5LLgqAHLgJHhYW3LPYS1RV5V5GDQyjQFaHHqEAq-M9AR6RfqbCRJcjMlSrcfdLcRAFaAKOrYX8mM1fiDF3qxs-Ga2iJI4dDiRzDSpxVItCT8FG2UO-xXZJmBiX9Hw38M6bPSZv9hZcc-kzr4ysQnYeK3z8vOsteHd_3BfJuhkfZ4Jn8wLRAUoBk3DRzkxtLXDpuIgYOLXEAy-_0sJBhw3TCY3JZ6j15SqPyQnraBUfBGzWHr_KaR90MJ_BgPaHDw" {
		t.Fatal("different N")
	}

	if k.E != "AQAB" {
		t.Fatal("different E")
	}
}

func TestDecodeJWK(t *testing.T) {
	decoded, err := jwt.DecodeJWK(jwt.JWKKey{
		Kty: "RSA",
		Alg: "RS256",
		N:   "pv4op2E1U8Gff9aip3PHsfvBYlI_mNlOiz7PMy40znRs6NEwe6uFyhPgxqx3uF6KHv1k2md9v43LSmb2pZdOZP5LLgqAHLgJHhYW3LPYS1RV5V5GDQyjQFaHHqEAq-M9AR6RfqbCRJcjMlSrcfdLcRAFaAKOrYX8mM1fiDF3qxs-Ga2iJI4dDiRzDSpxVItCT8FG2UO-xXZJmBiX9Hw38M6bPSZv9hZcc-kzr4ysQnYeK3z8vOsteHd_3BfJuhkfZ4Jn8wLRAUoBk3DRzkxtLXDpuIgYOLXEAy-_0sJBhw3TCY3JZ6j15SqPyQnraBUfBGzWHr_KaR90MJ_BgPaHDw",
		E:   "AQAB",
	})

	if err != nil {
		t.Fatal(err)
	}

	pub, _ := jwt.DecodeRSAPubKeyPem(pubPem)
	if pub.N.Cmp(decoded.N) != 0 {
		t.Fatal("different N")
	}
	if pub.E != decoded.E {
		t.Fatal("different E")
	}
}
