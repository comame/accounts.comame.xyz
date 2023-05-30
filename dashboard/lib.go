package main

import (
	"log"
	"runtime"
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

	if !ok {
		log.Printf("no_file_info:L0 %s", err)
		return err
	}

	log.Printf("%s:L%d %s", file, line, err)
	return err
}
