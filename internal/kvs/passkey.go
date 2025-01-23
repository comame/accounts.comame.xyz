package kvs

import "context"

func PasskeyChallenge_save(challenge, sessionID string) error {
	key := "PasskeyChallenge:" + sessionID
	if err := Set(context.Background(), key, challenge, 10*60); err != nil {
		return err
	}

	return nil
}

func PasskeyChallenge_get(sessionID string) (challenge string, err error) {
	key := "PasskeyChallenge:" + sessionID
	v, err := Get(context.Background(), key)
	if err != nil {
		return "", err
	}

	return v, nil
}

func PasskeyChallenge_delete(sessionID string) {
	key := "PasskeyChallenge:" + sessionID
	Del(context.Background(), key)
}
