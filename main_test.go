package main

import (
	"net/http/httptest"
	"testing"

	"github.com/comame/accounts.comame.xyz/tests"
)

func TestPerformScenarios(t *testing.T) {
	if testing.Short() {
		t.SkipNow()
		return
	}

	ts := httptest.NewServer(getAppHandler())

	scenarios, err := tests.GetScenarios()
	if err != nil {
		t.Fatal(err)
	}

	for _, scenario := range scenarios {
		tests.TestScenario(t, &scenario, ts)
	}
}
