package kvs

func KeyPasskeyChallenge(id string) string {
	return "PasskeyChallenge:" + id
}

func KeySigninSession(id string) string {
	return "SigninSession:" + id
}
