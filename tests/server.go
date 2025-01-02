package tests

import (
	"context"
	"log"
	"net/http"
	"net/http/httptest"
	"sync"
	"testing"
)

type interactiveServer struct {
	srv       *http.Server
	listening bool
	m         *sync.Mutex
	wg        *sync.WaitGroup
	ts        *httptest.Server
}

func NewInteractiveServer(ts *httptest.Server) *interactiveServer {
	return &interactiveServer{
		srv: &http.Server{
			Addr: ":8080",
		},
		listening: false,
		m:         &sync.Mutex{},
		wg:        &sync.WaitGroup{},
		ts:        ts,
	}
}

func (s *interactiveServer) Shutdown() {
	if !s.listening {
		s.srv.Shutdown(context.Background())
	}
}

func (s *interactiveServer) SetAssertion(t *testing.T, step *assertIncomingRequestStep, variables *map[string]string) {
	s.wg.Add(1)

	s.srv.Handler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		gotPath := r.URL.Path
		if len(r.URL.RawQuery) > 0 {
			gotPath += "?" + r.URL.RawQuery
		}
		expectedPath := capture(step.Path, gotPath, variables)

		if r.Method != step.Method {
			log.Printf("メソッドが異なる expected %s got %s", step.Method, r.Method)
			t.FailNow()
		}
		if gotPath != expectedPath {
			log.Printf("パスが異なる expected %s got %s", expectedPath, gotPath)
			t.FailNow()
		}

		for k, v := range step.AdditionalHeader {
			r.Header.Add(k, capture(v, v, variables))
		}
		s.ts.Config.Handler.ServeHTTP(w, r)

		s.wg.Done()
	})

	if !s.listening {
		go func() {
			log.Println("Start interactive server on :8080")
			if err := s.srv.ListenAndServe(); err != nil {
				if err == http.ErrServerClosed {
					log.Println("Shutdown interactive server")
					return
				}
				panic(err)
			}
		}()
		s.listening = true
	}

	log.Println("想定したリクエストを受け取るまで待機...")
	s.wg.Wait()
	log.Println("受け取ったので進行")
}
