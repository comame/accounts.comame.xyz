package kvs

import "context"

type kvsClient interface {
	Keys(ctx context.Context, pattern string) ([]string, error)
	Set(ctx context.Context, key string, value string, expireSec int64) error
	Get(ctx context.Context, key string) (string, error)
	Del(ctx context.Context, key string)
}

var cl kvsClient

func InitializeKVS(client kvsClient) {
	cl = client
}

func Keys(ctx context.Context, pattern string) ([]string, error) {
	v, err := cl.Keys(ctx, pattern)
	if err != nil {
		return nil, err
	}
	return v, nil
}

func Set(ctx context.Context, key string, value string, expireSec int64) error {
	if err := cl.Set(ctx, key, value, expireSec); err != nil {
		return err
	}
	return nil
}

func Get(ctx context.Context, key string) (string, error) {
	v, err := cl.Get(ctx, key)
	if err != nil {
		return "", err
	}
	return v, nil
}

func Del(ctx context.Context, key string) {
	cl.Del(ctx, key)
}
