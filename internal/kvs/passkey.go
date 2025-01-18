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
