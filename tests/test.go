package tests

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"os"
	"reflect"
	"strings"
	"sync"
	"testing"

	"github.com/comame/accounts.comame.xyz/db"
)

func TestScenario(t *testing.T, s *scenario, ts *httptest.Server) {
	log.Println(s.Name)

	variables := make(map[string]string)

	testPrepare(t)

	for i, step := range s.Steps {
		switch v := step.(type) {
		case httpRequestStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testHttpRequestStep(t, &v, ts, &variables)
		case sqlStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testSQLStep(t, &v)
		case timeFreezeStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testTimeFreezeStep(t, &v)
		case assertIncomingRequestStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testAssertIncomingRequestStep(t, &v, ts, &variables)
		case printStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testPrintStep(t, &v, &variables)
		case interactiveStep:
			if os.Getenv("INTERACTIVE") == "" {
				log.Printf("skip interactive test")
				log.Println()
				return
			}
		default:
			log.Println("Stepのキャストに失敗")
			t.FailNow()
		}
	}

	log.Println("success")
	log.Println()
}

func testHttpRequestStep(t *testing.T, s *httpRequestStep, ts *httptest.Server, variables *map[string]string) {
	var reqBody io.Reader
	if s.ReqBody != "" {
		reqBody = strings.NewReader(capture(s.ReqBody, s.ReqBody, variables))
	}
	req, _ := http.NewRequest(s.ReqMethod, ts.URL+s.ReqPath, reqBody)
	for k, v := range s.ReqHeaders {
		v = capture(v, v, variables)
		req.Header[k] = []string{v}
	}

	http.DefaultClient.CheckRedirect = func(req *http.Request, via []*http.Request) error {
		return http.ErrUseLastResponse
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
		v = capture(v, gv[0], variables)
		if !reflect.DeepEqual(gv, []string{v}) {
			log.Printf("header %s expected %v got %v", k, v, gv)
			t.FailNow()
			return
		}
	}

	expectBody := strings.TrimSpace(string(s.ResBody))
	if expectBody == "" {
		return
	}

	resBody, _ := io.ReadAll(res.Body)
	gotBody := strings.TrimSpace(string(resBody))

	expectBody = capture(expectBody, gotBody, variables)

	if expectBody != gotBody {
		log.Println("body expected:")
		fmt.Println(expectBody)
		log.Println("body got:")
		fmt.Println(gotBody)
		t.FailNow()
		return
	}
}

func testSQLStep(t *testing.T, s *sqlStep) {
	if _, err := db.Conn().Exec(s.Query); err != nil {
		log.Println("DBがエラーを返した")
		log.Println(err)
		t.FailNow()
	}
}

func testTimeFreezeStep(_ *testing.T, s *timeFreezeStep) {
	setTimeFreeze(s.Datetime)
}

func testAssertIncomingRequestStep(t *testing.T, s *assertIncomingRequestStep, ts *httptest.Server, variables *map[string]string) {
	srv := &http.Server{
		Addr: ":8080",
	}

	var wg sync.WaitGroup
	var mut sync.Mutex

	failed := false

	srv.Handler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !mut.TryLock() {
			// 複数回リクエストを受け付けたときは 500 を返して処理しない
			w.WriteHeader(http.StatusInternalServerError)
			return
		}

		expectedPath := capture(s.Path, r.URL.Path, variables)

		if r.Method != s.Method {
			failed = true
			log.Printf("メソッドが異なる expected %s got %s", s.Method, r.Method)
		}
		if r.URL.Path != expectedPath {
			failed = true
			log.Printf("パスが異なる expected %s got %s", expectedPath, r.URL.Path)
		}
		if failed {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}

		for k, v := range s.AdditionalHeader {
			r.Header.Add(k, capture(v, v, variables))
		}
		ts.Config.Handler.ServeHTTP(w, r)

		// TODO: 後で消す
		failed = true

		// リクエストを受け取ったら進行する
		wg.Done()
	})

	go func() {
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			// テストの Goroutine ではないので、t.FailNow ではなく panic する
			panic(err)
		}
	}()

	wg.Add(1)
	wg.Wait()

	if failed {
		t.FailNow()
	}
}

func testPrintStep(_ *testing.T, s *printStep, variables *map[string]string) {
	log.Println(capture(s.Message, s.Message, variables))
}

func testPrepare(t *testing.T) {
	if err := setup(); err != nil {
		log.Println(err)
		t.FailNow()
	}
}
