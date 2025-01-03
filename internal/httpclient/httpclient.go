package httpclient

import (
	"net/http"
)

func Client() *http.Client {
	if c != nil {
		return c
	}

	return http.DefaultClient
}

var c *http.Client

func SetClientForTest(cl *http.Client) {
	c = cl
}
