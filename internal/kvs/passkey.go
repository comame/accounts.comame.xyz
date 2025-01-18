package kvs

import "context"

func PasskeyChallenge_save(
	challenge string,
	userID string,
) error {
	key := "PasskeyChallenge:" + challenge
	if err := Set(context.Background(), key, userID, 10*60); err != nil {
		return err
	}

	return nil
}

func PasskeyChallenge_get(
	challenge string,
) (userID string, err error) {
	key := "PasskeyChallenge:" + challenge
	v, err := Get(context.Background(), key)
	if err != nil {
		return "", err
	}

	return v, nil
}

func PasskeyChallenge_delete(challenge string) {
	key := "PasskeyChallenge:" + challenge
	Del(context.Background(), key)
}
