package tests

import (
	"io"
	"log"
	"net/http"
	"strconv"
	"strings"

	"github.com/comame/accounts.comame.xyz/internal/httpclient"
)

type mockTransport struct {
	mocks     []httpMock
	variables *map[string]string
}

type httpMock struct {
	Method  string
	URL     string
	Headers map[string]string
	Body    string
	Res     http.Response
}

func (t *mockTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	// リクエストとモックを突き合わせ、違っていたら Fatal で終了させてテストを失敗させる
	if len(t.mocks) == 0 {
		log.Fatalf("モックされていないリクエストを行った %v", req)
	}
	m := t.mocks[0]
	if len(t.mocks) == 1 {
		t.mocks = nil
	}
	if len(t.mocks) >= 2 {
		t.mocks = t.mocks[1:]
	}

	if req.Method != m.Method {
		log.Fatalf("想定されたメソッドと違う expect %s got %s", m.Method, req.Method)
	}

	gotURL := req.URL.String()
	expectURL := m.URL
	expectURL = capture(expectURL, gotURL, t.variables)
	if gotURL != expectURL {
		log.Fatalf("想定されたURLと違う expect %s got %s", expectURL, gotURL)
	}

	for k, v := range m.Headers {
		if rh := req.Header.Get(k); rh != v {
			log.Fatalf("想定されたヘッダと違う key=%s expect %s got %s", k, v, rh)
		}
	}

	if m.Body != "" {
		if req.Body == nil {
			log.Fatalf("リクエストボディがない")
		}
		b, _ := io.ReadAll(req.Body)
		expectBody := m.Body
		gotBody := string(b)
		expectBody = capture(expectBody, gotBody, t.variables)
		if expectBody != gotBody {
			log.Fatalf("リクエストボディが違う expect %s got %s", expectBody, gotBody)
		}
	}

	return &m.Res, nil
}

func createMockResponse(status int, header map[string]string, body string) http.Response {
	var r http.Response
	r.StatusCode = status
	r.Status = strconv.Itoa(status)

	for k, v := range header {
		r.Header.Set(k, v)
	}

	r.Body = io.NopCloser(strings.NewReader(body))
	r.ContentLength = int64(len([]byte(body)))

	return r
}

var tr *mockTransport

func setMockRequestForTest(m httpMock, variables *map[string]string) {
	if tr == nil {
		tr = &mockTransport{
			// variables は動的に変わるのでポインタを引き回す...
			variables: variables,
		}
		httpclient.SetClientForTest(&http.Client{
			// tr はグローバル変数へのポインタなので、複数回呼び出したときに正しくモックが増えるはず
			Transport: tr,
		})
	}

	tr.mocks = append(tr.mocks, m)
}
