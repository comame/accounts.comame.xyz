package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"reflect"
	"strings"
	"testing"

	"github.com/comame/accounts.comame.xyz/tests"
)

func TestPerformeScenarios(t *testing.T) {
	if testing.Short() {
		t.SkipNow()
		return
	}

	ts := httptest.NewServer(getAppHandler())

	scenarios, err := tests.GetScenarios()
	if err != nil {
		t.Fatal(err)
	}

	// TODO: 複数のリクエスト・レスポンスを繰り返せるようにする
	for _, scenario := range scenarios {
		var reqBody io.Reader
		if scenario.ReqBody != "" {
			reqBody = strings.NewReader(scenario.ReqBody)
		}
		req, _ := http.NewRequest(scenario.ReqMethod, ts.URL+scenario.ReqPath, reqBody)
		for k, v := range scenario.ReqHeaders {
			req.Header[k] = v
		}

		res, _ := http.DefaultClient.Do(req)

		if res.StatusCode != scenario.ResStatus {
			log.Println(scenario.Name)
			log.Printf("status expected %d got %d", scenario.ResStatus, res.StatusCode)
			t.FailNow()
			return
		}
		for k, v := range scenario.ResHeaders {
			gv, ok := res.Header[k]
			if !ok {
				log.Println(scenario.Name)
				log.Printf("header not present %s", k)
				t.FailNow()
				return
			}
			if !reflect.DeepEqual(gv, v) {
				log.Println(scenario.Name)
				log.Printf("header %s expected %v got %v", k, v, gv)
				t.FailNow()
				return
			}
		}

		resBody, _ := io.ReadAll(res.Body)
		if strings.TrimSpace(string(resBody)) != strings.TrimSpace(scenario.ResBody) {
			log.Println(scenario.Name)
			log.Println("body expected:")
			fmt.Println(scenario.ResBody)
			log.Println("body got:")
			fmt.Println(string(resBody))
			t.FailNow()
			return
		}
	}

}
