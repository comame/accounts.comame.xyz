package main

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
