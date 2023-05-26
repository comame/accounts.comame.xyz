package vks

import (
	"context"
	"fmt"
	"time"

	readenv "github.com/comame/readenv-go"
	"github.com/redis/go-redis/v9"
)

type env_t struct {
	Host string `env:"REDIS_HOST"`
}

const prefix = "dash.accounts.comame.xyz:"

var client *redis.Client
var env env_t

func init() {
	readenv.Read(&env)
	client = redis.NewClient(&redis.Options{
		Addr: env.Host + ":6379",
	})
}

func Keys(ctx context.Context, pattern string) ([]string, error) {
	keys, err := client.Keys(ctx, prefix+pattern).Result()
	if err != nil {
		return nil, err
	}

	var trimed []string
	for _, key := range keys {
		prefixLen := len(prefix)
		if len(key) <= prefixLen {
			return nil, fmt.Errorf("invalid format key")
		}
		trimed = append(trimed, key[len(prefix):])
	}

	return trimed, nil
}

func Set(ctx context.Context, key string, value string, expireSec uint16) error {
	if expireSec <= 0 {
		return fmt.Errorf("expireSec must larger than 0")
	}

	err := client.Set(ctx, prefix+key, value, time.Duration(expireSec)*time.Second).Err()
	if err != nil {
		return err
	}

	return nil
}

func Get(ctx context.Context, key string) (string, error) {
	v, err := client.Get(ctx, prefix+key).Result()
	if err != nil {
		return "", err
	}
	return v, nil
}
