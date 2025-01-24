package ceremony

import (
	"fmt"
	"io"
	"net/http"
)

var (
	messageBadRequest        = "bad_request"
	messageInvalidCredential = "invalid_credential"
	messageUnauthorized      = "unauthorized"
)

func responseError(w http.ResponseWriter, message string) {
	w.WriteHeader(http.StatusBadRequest)
	io.WriteString(w, fmt.Sprintf(`{ "error": "%s" }`, message))
}
