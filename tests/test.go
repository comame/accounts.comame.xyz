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
		case timeFreezeStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testTimeFreezeStep(t, &v)
		case prepareKeyPairStep:
			log.Printf("step %d %s", i, v.StepDescription)
			testPrepareKeyPairStep(t, &v)
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

func testPrepareKeyPairStep(t *testing.T, _ *prepareKeyPairStep) {
	if _, err := db.Conn().Exec(`
insert into rsa_keypair
(id, public, private, kid)
values
(
    1,
"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAsicr/hIYRFhuKCQ+NBax
ZeNj3/yTcqu6PKVC7+9X6R74ITXTnweArSaoASKNyaWmanuEGvNmE4cZm3/4BMzJ
2YW6V5X6qj5xM7tolYAZ2VCCCcwPLbK54q3tgrzPZtPl/QpiJ4hCx17ajBvemOfS
OCXYoPy+IFL2ISiReqibiQJU7amZvVb+KmRnfCE2az+3Cj8D+fEd1SZQ+BWcDb36
L6Iw/1mvP5F58GrnkSwOQ2UvZESwR74DhlP2AoeTUw/lUbmgBLhz/P0Clk0RBuLE
HR6/VPLQVwa7PRSsawJKAAA1Ha9SCb9fWD6AJ6Q6pJ5taAqvwGe1w+wZOPMyTv0g
YwIDAQAB
-----END PUBLIC KEY-----",
"-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQEAsicr/hIYRFhuKCQ+NBaxZeNj3/yTcqu6PKVC7+9X6R74ITXT
nweArSaoASKNyaWmanuEGvNmE4cZm3/4BMzJ2YW6V5X6qj5xM7tolYAZ2VCCCcwP
LbK54q3tgrzPZtPl/QpiJ4hCx17ajBvemOfSOCXYoPy+IFL2ISiReqibiQJU7amZ
vVb+KmRnfCE2az+3Cj8D+fEd1SZQ+BWcDb36L6Iw/1mvP5F58GrnkSwOQ2UvZESw
R74DhlP2AoeTUw/lUbmgBLhz/P0Clk0RBuLEHR6/VPLQVwa7PRSsawJKAAA1Ha9S
Cb9fWD6AJ6Q6pJ5taAqvwGe1w+wZOPMyTv0gYwIDAQABAoIBAG2KqoExzRwRJ8Kk
7l6G6ZNVqy6pllw2/W+Wyj7P80UTVszM1Q9+xH8zOrBf98DaiyYERqlvqf8t3fAA
UpdY+HA4yuhZ/uQ5Os/tVxQ9zScTWrH9eAPIVoXsHhN6VyjJ+CuL++iE31LJnyXx
aQCp4lfF5ZqvbZRgjpi64iEClYg7D+LzBuBNEo7RjUISEN0EbhOQ/rbpGUgw1zrI
M9o8Jk8IzkBKbJ7OkQXcYA9qQs+3q89tqBRejzKhNewfWc3nr7wuEMSRfDH7x41Y
ndAvmjpPjw/dgfkUZKT7pFYWY0IAtBqwCD21Dhn5L/2oltVRI8hR4gYxx+L9Wmjp
tdO0flkCgYEAwVyQcMSdikyMLh1BxanWrG1tYUmi1oOj8pC63D+wRa9eCWE9tJ65
1dr6o7tW8kz+yQANELrgI1VHQLR+c7Q0eIBOMXxiDiebeMpGtbx6IUYVpKsZqXg1
8FdmGaaV31B776QNucW6U2+0vywTGlJiDDPASuUC0oPIaA6Wk0NmwwcCgYEA691d
U7e6nBZIUB26lO2RJXJrbew1Mkamx8/x2odKEpQ1AF6E/4s91WrHqU90kfdp07+b
KzrSe0TzUZ01j+E41uEgB8WB3YfTaMsQgeXngX6ZYyPlLCO1r9Gsz8GURa8q4uoy
N6jtGpouMnOZlckhFlov9smRvPhH8fMvzB9PlMUCgYBuvDAMJM2EEmqFTkQIi0dh
4BkwChezehg+JhydXev5PIFCJepMskoC6zF26ybUBLw1KE5TMnKCSahQqg1w/da+
29vsAyu0p4ImHtF36sSWoahrcYF0yF87kRHrxrc1+MXBa9ZgeZhHiEWe5gLapCt6
iXiqa5S+MrJmxVP+ai9DqQKBgHPCqHxfLypOUV1oydswIc20M3+2r4EmZdKpf3UW
c0ddEApHWZUmHMny5110jqzZNkpjvt9ftlAjzhvfQZuFGWV1BkhqKku0zxCeoVJv
qMjIfrXGt0KLoC9ThDJPOttclnraIJ1qvjwRMd03GUkHdsLGrsW7tlh9rqnUBkBz
mZZVAoGAaqAMBgQpZ/ZGKwp45cNXd6TVCBfoaS5a+XB19Yjf/ELTrxP699zoPAwa
co3x+5Z7BeiG7vEg4D+HMxgaxQOQysklaF98knlLz2YGz3KWPqG4tPgiyPvakxlL
uIyrJoprYT/ZypCvUiEbbFDHOdeHEeFUHdOPltTEUZeQteq27IQ=
-----END RSA PRIVATE KEY-----",
"TNkhLzSn"
)
on duplicate key update
    public = values(public),
    private = values(private),
    kid = values(kid)
;
	`); err != nil {
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
