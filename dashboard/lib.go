package main

import (
	"log"
	"runtime"
	"strings"
)

func mapValues[K comparable, V any](m map[K]V) []V {
	var values []V

	for _, v := range m {
		values = append(values, v)
	}

	return values
}

func derefSlice[T any](s []*T) []T {
	var result []T

	for _, v := range s {
		result = append(result, *v)
	}

	return result
}

func logErr(err error) error {
	_, file, line, ok := runtime.Caller(1)
	logger := log.New(log.Default().Writer(), log.Default().Prefix(), log.LstdFlags)

	if !ok {
		logger.Printf("no_file_info:L0: %s", err)
		return err
	}

	paths := strings.Split(file, "/")
	logger.Printf("%s:%d: %s", paths[len(paths)-1], line, err)
	return err
}
