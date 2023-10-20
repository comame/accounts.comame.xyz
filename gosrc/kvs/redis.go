package kvs

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
)

var _prefix string
var _client *redis.Client

func Initialize(prefix, host string) {
	_prefix = prefix
	_client = redis.NewClient(&redis.Options{
		Addr: host,
	})
}

func Keys(ctx context.Context, pattern string) ([]string, error) {
	keys, err := _client.Keys(ctx, _prefix+pattern).Result()
	if err != nil {
		return nil, err
	}

	var trimed []string
	for _, key := range keys {
		prefixLen := len(_prefix)
		if len(key) <= prefixLen {
			return nil, fmt.Errorf("invalid format key")
		}
		trimed = append(trimed, key[len(_prefix):])
	}

	return trimed, nil
}

func Set(ctx context.Context, key string, value string, expireSec uint16) error {
	if expireSec <= 0 {
		return fmt.Errorf("expireSec must larger than 0")
	}

	err := _client.Set(ctx, _prefix+key, value, time.Duration(expireSec)*time.Second).Err()
	if err != nil {
		return err
	}

	return nil
}

func Get(ctx context.Context, key string) (string, error) {
	v, err := _client.Get(ctx, _prefix+key).Result()
	if err != nil {
		return "", err
	}
	return v, nil
}

func Del(ctx context.Context, key string) {
	_client.Del(ctx, _prefix+key)
}
