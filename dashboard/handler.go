package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
)

func handleIndex(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleSignin(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
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

	// TODO: 認可ができるようになったら、正しくレスポンスを返す

	log.Println(string(json))

	io.WriteString(w, "unimplemented\n")
}

func responseError(w http.ResponseWriter, err error) {
	log.Println(err)

	// TODO: エラーレスポンスを実装する
	w.WriteHeader(http.StatusBadRequest)
}
