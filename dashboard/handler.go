package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

type tokenRequest struct {
	Token string `json:"token"`
}

func handleIndex(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleSignin(w http.ResponseWriter, r *http.Request) {
	url, err := createAuthUrl(r.Context())
	if err != nil {
		responseError(w, err)
		return
	}

	w.Header().Add("Location", url)
	w.WriteHeader(http.StatusFound)
}

func handleCallback(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpList(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpCreate(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpUpdatesecret(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpDelete(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpRedirecturiAdd(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleRpRedirecturiRemove(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserList(w http.ResponseWriter, r *http.Request) {
	var body tokenRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(w, body.Token) {
		return
	}

	users, err := listUser(r.Context())
	responseJsonData(w, r, users, err)
}

func handleUserCreate(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserDelete(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserPasswordChange(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserPasswordRemove(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserSessionList(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserSessionRevoke(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserAuthenticationList(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleNotFound(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "not found\n")
}

// data は json.Unmarshal の第 2 引数
func parseBody(w http.ResponseWriter, r *http.Request, data interface{}) (ok bool) {
	bytes, err := io.ReadAll(r.Body)
	if err != nil {
		responseError(w, err)
		return false
	}

	if err := json.Unmarshal(bytes, data); err != nil {
		responseError(w, err)
		return false
	}

	return true
}

func authorized(token string) bool {
	// TODO: 実装する
	return true
}

func authorizedOrReturn(w http.ResponseWriter, token string) (ok bool) {
	if !authorized(token) {
		responseError(w, fmt.Errorf("unauthorized"))
		return false
	}
	return true
}

func responseJsonData(w http.ResponseWriter, r *http.Request, data interface{}, err error) {
	if err != nil {
		responseError(w, err)
		return
	}

	json, err := json.Marshal(data)
	if err != nil {
		responseError(w, err)
		return
	}

	fmt.Fprintln(w, string(json))
}

func responseError(w http.ResponseWriter, err error) {
	log.Println(err)

	// TODO: エラーレスポンスを実装する
	w.WriteHeader(http.StatusBadRequest)
}
