package kvs

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
)

type redisClient struct {
	cl *redis.Client
	pf string
}

func InitializeRedis(prefix, host string) {
	c := redisClient{
		cl: redis.NewClient(&redis.Options{
			Addr: host,
		}),
		pf: prefix,
	}
	InitializeKVS(&c)
}

func (rc *redisClient) Keys(ctx context.Context, pattern string) ([]string, error) {
	keys, err := rc.cl.Keys(ctx, rc.pf+pattern).Result()
	if err != nil {
		return nil, err
	}

	var trimed []string
	for _, key := range keys {
		prefixLen := len(rc.pf)
		if len(key) <= prefixLen {
			return nil, fmt.Errorf("invalid format key")
		}
		trimed = append(trimed, key[len(rc.pf):])
	}

	return trimed, nil
}

func (rc *redisClient) Set(ctx context.Context, key string, value string, expireSec int64) error {
	if expireSec <= 0 {
		return fmt.Errorf("expireSec must larger than 0")
	}

	err := rc.cl.Set(ctx, rc.pf+key, value, time.Duration(expireSec)*time.Second).Err()
	if err != nil {
		return err
	}

	return nil
}

func (rc *redisClient) Get(ctx context.Context, key string) (string, error) {
	v, err := rc.cl.Get(ctx, rc.pf+key).Result()
	if err != nil {
		return "", err
	}
	return v, nil
}

func (rc *redisClient) Del(ctx context.Context, key string) {
	rc.cl.Del(ctx, rc.pf+key)
}
