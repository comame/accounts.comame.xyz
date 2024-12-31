package tests

import (
	"testing"
)

func Testキャプチャができる(t *testing.T) {
	v := make(map[string]string)
	ret := capture(
		"aaa{{foo}}aaa{{bar}}",
		"aaafoo_valueaaabar_value",
		&v,
	)

	if len(v) != 2 {
		t.Fail()
	}
	if v["foo"] != "foo_value" {
		t.Fail()
	}
	if v["bar"] != "bar_value" {
		t.Fail()
	}
	if ret != "aaafoo_valueaaabar_value" {
		t.Fail()
	}
}

func Test置換ができる(t *testing.T) {
	v := map[string]string{
		"foo": "foo_value",
		"bar": "bar_value",
	}
	ret := capture(
		"aaa((foo))aaa((bar))",
		"some random value",
		&v,
	)

	if len(v) != 2 {
		t.Fail()
	}
	if v["foo"] != "foo_value" {
		t.Fail()
	}
	if v["bar"] != "bar_value" {
		t.Fail()
	}
	if ret != "aaafoo_valueaaabar_value" {
		t.Fail()
	}
}

func Test両方ができる(t *testing.T) {
	v := map[string]string{
		"bar": "bar_value",
	}
	ret := capture(
		"aaa{{foo}}aaa((bar))",
		"aaafoo_valueaaabar_value",
		&v,
	)

	if len(v) != 2 {
		t.Fail()
	}
	if v["foo"] != "foo_value" {
		t.Fail()
	}
	if v["bar"] != "bar_value" {
		t.Fail()
	}
	if ret != "aaafoo_valueaaabar_value" {
		t.Fail()
	}
}
