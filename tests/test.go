package tests

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"reflect"
	"strings"
	"testing"
)

func TestScenario(t *testing.T, s *scenario, ts *httptest.Server) {
	log.Println(s.Name)

	for i, step := range s.Steps {
		switch v := step.(type) {
		case httpRequestStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testHttpRequestStep(t, &v, ts)
		default:
			log.Println("Stepのキャストに失敗")
			t.FailNow()
		}
	}

	log.Println("success")
	log.Println()
}

func testHttpRequestStep(t *testing.T, s *httpRequestStep, ts *httptest.Server) {
	var reqBody io.Reader
	if s.ReqBody != "" {
		reqBody = strings.NewReader(s.ReqBody)
	}
	req, _ := http.NewRequest(s.ReqMethod, ts.URL+s.ReqPath, reqBody)
	for k, v := range s.ReqHeaders {
		req.Header[k] = v
	}

	res, _ := http.DefaultClient.Do(req)

	if res.StatusCode != s.ResStatus {
		log.Printf("status expected %d got %d", s.ResStatus, res.StatusCode)
		t.FailNow()
		return
	}
	for k, v := range s.ResHeaders {
		gv, ok := res.Header[k]
		if !ok {
			log.Printf("header not present %s", k)
			t.FailNow()
			return
		}
		if !reflect.DeepEqual(gv, v) {
			log.Printf("header %s expected %v got %v", k, v, gv)
			t.FailNow()
			return
		}
	}

	resBody, _ := io.ReadAll(res.Body)
	if strings.TrimSpace(string(resBody)) != strings.TrimSpace(s.ResBody) {
		log.Println("body expected:")
		fmt.Println(s.ResBody)
		log.Println("body got:")
		fmt.Println(string(resBody))
		t.FailNow()
		return
	}
}
