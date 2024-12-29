package tests

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"reflect"
	"regexp"
	"strings"
	"testing"

	"github.com/comame/accounts.comame.xyz/db"
)

func TestScenario(t *testing.T, s *scenario, ts *httptest.Server) {
	log.Println(s.Name)

	variables := make(map[string]string)

	for i, step := range s.Steps {
		switch v := step.(type) {
		case httpRequestStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testHttpRequestStep(t, &v, ts, &variables)
		case sqlStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testSQLStep(t, &v)
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

var capturePattern = regexp.MustCompile(`{{([a-zA-Z0-9]+)}}`)
var embedPattern = regexp.MustCompile(`\(\(([a-zA-Z0-9]+)\)\)`)

// TODO: 別ファイルに移してテストも書く...
func capture(template, target string, variables *map[string]string) string {
	m := capturePattern.FindAllStringSubmatchIndex(template, -1)
	patternStr := ""
	last := 0
	var names []string
	for _, match := range m {
		patternStr += regexp.QuoteMeta(template[last:match[0]])
		patternStr += "([a-zA-Z0-9-_=\\.]+)"
		last = match[1]
		names = append(names, template[match[2]:match[3]])
	}
	patternStr += regexp.QuoteMeta(template[last:])

	pattern, err := regexp.Compile(patternStr)
	if err != nil {
		return template
	}
	ms := pattern.FindStringSubmatch(target)

	if len(ms) >= 2 {
		for i, name := range names {
			(*variables)[name] = ms[i+1]
		}
	}

	replaced := ""
	last = 0
	for _, match := range m {
		replaced += template[last:match[0]]
		v, ok := (*variables)[template[match[2]:match[3]]]
		if ok {
			replaced += v
		} else {
			replaced += template[match[2]:match[3]]
		}
		last = match[1]
	}
	replaced += template[last:]

	me := embedPattern.FindAllStringSubmatchIndex(template, -1)
	if len(me) == 0 {
		return replaced
	}
	embed := ""
	last = 0
	for _, match := range me {
		embed += template[last:match[0]]

		v, ok := (*variables)[template[match[2]:match[3]]]
		if ok {
			embed += v
		} else {
			embed += template[match[0]:match[1]]
		}

		last = match[1]
	}
	embed += template[last:]
	return embed
}
