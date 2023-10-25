package jwt_test

import (
	"log"
	"testing"

	"github.com/comame/accounts.comame.xyz/jwt"
)

const privPem = `-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEApv87doY8JQGmpwhiDFMlrIOx5TVbIKkN0Ly2Tdz2ns1gmHqP
XKQ6Bp/ibITmRUe0FlqlK+e3SH9L9YMPrzhYh21HLw4AAcUGBztIFE5GHoWD8wrh
bU1AUDQRAmnz/OIY4SnoEzYlmvsNK1Paml2i032Y4D5/js602VBXKEk7DeR3cqen
mVMbmbFzJ0wKWijIiiG2Xf/N3YflD5lm2wfqKUS6uirxt7RpIEdBH9nD8pWApAsn
Wstbjk7yyOYxREW0wsFfYeBNIXVmdxJAT0BULniQX1l6LWO+haOkAIGq9IL+ZCgh
XJSW/4pxr2DTIZkQqiQ3Iy/IBuKUcQ3fSSAacwIDAQABAoIBAB6z1CnhWhNvr0UM
XIJpkwaZm85JNfzWN+0FlPrRwiHGHplKByuAIg7vvEA5WuCn4yss9SsSSfcxODTm
KK8NS/FyHOFA0K2CnK4drw3Uqj2YTH8VpAZlaoqUWAA6nJoEs5BKFRnbHaTVvUEX
BJzW1EXP1Jh9xRPrWwKNK8kDVbEi4MB2V2flpI1soYUPojLErEe+6ZUBcRzDMFVL
mHqSHVn6QagoUp0H2fpiqLlmuDgmlJWvIrQ8w1jTOoOLuhp5Un+8OWwnmY4qGunR
JwIKQmVpkn5GJfhRATNxLhYWFCJHZRKthlaZgXzJv/umPGVCB14yKoVYCkG9Tn8T
X0UCW+kCgYEA3bR0pEoqXMOcc78pRaNQFAK3KYcxY/4uaijgSB1X5rfMTTBegwlq
frUP1BqEB7H66Y9EfCgZAKULBOyGpNFjfwQ6AQUcncI6rF0cwo/Nm/WcK32ceRbH
Ui+5xBlebn4bWH+qvCv+5Sn2k2XT86C/gMT9KQ4AtUPlz1KNGCC+Rs0CgYEAwNRW
EpSXiWvCoDqJkzjxKLJENuCO1197MkUqaWBseORxqHI8ji6WQSRuPV0Ndmro+XJ6
uai2JVYJHIXqcehTXqn3zVivQIMqIbj8+UV3QdJpHLXfKd8zU53IW4fzEMWfQ5q1
gdPYxKx7T+fPtr3svAT9ZYBnR2pVrpGDeAxzZj8CgYBC+lxusbVAlxvx3+OBFUiA
8x0Qn7YtJkIxci3nu22t1wMSorU60yJoKx0gD+6pQHy0CMNA2wBKsJG2qBo5OsTb
P5SicD/n7SC6p9qjcLxGDJpkjSszbc1DqAVwF9XufYyXXOJgvM3hv56tgwrYREz2
gwyUVZWjLWMFEkRr8KBrVQKBgQC/34l2G51eBHg5b4YNlI+C6z+tS21XKY8wyloV
WPkWolnmPyW6ZOjmERYQwVLwDhmcfSlZLGNya1XNqANNLNwoSgBluGVKUDnQLH6s
m067lF87Tk2tIIe2ID6JtZFLkxmS75LEiMQdj3N0YznwoLO7s0thgI9EJK71cZ3c
CqeZiQKBgQCPx8ENPdMnQ4YHTPJ7mIyOce5TgdyzHZ7gbqi+y3NBKklrThwr1xGT
DuB2KBnSX10aWU8UVDdBmcyXMwcoVoMk1Xwa45T+E4WYRabr2GLvMQELEht/pKj1
kQMX7StAjNUQ78VsH9hXTmn5bNQOG9gO6RHSbB8Zy6gg0h+xVlDXgg==
-----END RSA PRIVATE KEY-----`

const pubPem = `-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApv87doY8JQGmpwhiDFMl
rIOx5TVbIKkN0Ly2Tdz2ns1gmHqPXKQ6Bp/ibITmRUe0FlqlK+e3SH9L9YMPrzhY
h21HLw4AAcUGBztIFE5GHoWD8wrhbU1AUDQRAmnz/OIY4SnoEzYlmvsNK1Paml2i
032Y4D5/js602VBXKEk7DeR3cqenmVMbmbFzJ0wKWijIiiG2Xf/N3YflD5lm2wfq
KUS6uirxt7RpIEdBH9nD8pWApAsnWstbjk7yyOYxREW0wsFfYeBNIXVmdxJAT0BU
LniQX1l6LWO+haOkAIGq9IL+ZCghXJSW/4pxr2DTIZkQqiQ3Iy/IBuKUcQ3fSSAa
cwIDAQAB
-----END PUBLIC KEY-----`

const idToken = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpc3MiOiJpc3N1ZXIiLCJzdWIiOiJzdWJzY3JpYmVyIiwiYXVkIjoiYXVkaWVuY2UiLCJleHAiOjAsImlhdCI6MCwibm9uY2UiOiJub25jZSIsInJvbGVzIjpudWxsfQ.J6iUjY4h8gE7nbkCpFZR3QfoRKdv1viU2AisKrxFiTHmZ3q5GhPbckcFZB2ntUGuD1jZf64DQfkm3T7sxe-T_4WvniyrTcS4ms7edurfMMIbXJNCoT-nsPsDxLCbwwoFJwTfRHBZ6gsludWJTQIMT2VlHsuvDiouYUH9F_RxY8FRx4jgDg01IopDontWCLyh7DRMyR9ITbzCOidLkE8jasiBAQFHdkFIej1x_hFxS-pvtDqNzXjQTrvIqMa6fxW35jCT2MGTr4SpQjYGCoOJvuPcihGr9cTpZgwKl7-Bf3ahmAcaiixsgY-Oyhd9PKKWDrko8bKIcJw_n_Lv6T51aA"

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

	log.Println(got)

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

	if k.N != "pv87doY8JQGmpwhiDFMlrIOx5TVbIKkN0Ly2Tdz2ns1gmHqPXKQ6Bp_ibITmRUe0FlqlK-e3SH9L9YMPrzhYh21HLw4AAcUGBztIFE5GHoWD8wrhbU1AUDQRAmnz_OIY4SnoEzYlmvsNK1Paml2i032Y4D5_js602VBXKEk7DeR3cqenmVMbmbFzJ0wKWijIiiG2Xf_N3YflD5lm2wfqKUS6uirxt7RpIEdBH9nD8pWApAsnWstbjk7yyOYxREW0wsFfYeBNIXVmdxJAT0BULniQX1l6LWO-haOkAIGq9IL-ZCghXJSW_4pxr2DTIZkQqiQ3Iy_IBuKUcQ3fSSAacw" {
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
		N:   "pv87doY8JQGmpwhiDFMlrIOx5TVbIKkN0Ly2Tdz2ns1gmHqPXKQ6Bp_ibITmRUe0FlqlK-e3SH9L9YMPrzhYh21HLw4AAcUGBztIFE5GHoWD8wrhbU1AUDQRAmnz_OIY4SnoEzYlmvsNK1Paml2i032Y4D5_js602VBXKEk7DeR3cqenmVMbmbFzJ0wKWijIiiG2Xf_N3YflD5lm2wfqKUS6uirxt7RpIEdBH9nD8pWApAsnWstbjk7yyOYxREW0wsFfYeBNIXVmdxJAT0BULniQX1l6LWO-haOkAIGq9IL-ZCghXJSW_4pxr2DTIZkQqiQ3Iy_IBuKUcQ3fSSAacw",
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
