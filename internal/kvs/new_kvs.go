package kvs

import (
	"context"
	"encoding/json"
	"fmt"
)

func Set2[T any](ctx context.Context, key string, value T, expireSec int64) error {
	if expireSec < 0 {
		return fmt.Errorf("expireSec (%d) < 0", expireSec)
	}

	valueJSON, err := json.Marshal(value)
	if err != nil {
		return err
	}

	if err := cl.Set(ctx, key, string(valueJSON), expireSec); err != nil {
		return err
	}

	return nil
}

func Get2[T any](ctx context.Context, key string) (T, error) {
	var ret T

	v, err := cl.Get(ctx, key)
	if err != nil {
		return ret, err
	}

	if err := json.Unmarshal([]byte(v), &ret); err != nil {
		return ret, err
	}

	return ret, nil
}
