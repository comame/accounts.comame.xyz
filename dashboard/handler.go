package main

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"io/fs"
	"log"
	"net/http"
)

type errorResponse struct {
	Error string `json:"message"`
}

type tokenRequest struct {
	Token string `json:"token"`
}

func handleStatic(w http.ResponseWriter, r *http.Request) {
	public, err := fs.Sub(static, "web/dist")
	if err != nil {
		panic(err)
	}

	server := http.FileServer(http.FS(public))
	strip := http.StripPrefix("/dash", server)
	strip.ServeHTTP(w, r)
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
	q := r.URL.Query()
	token, err := callbackAndIssueToken(r.Context(), q.Get("state"), q.Get("code"))
	if err != nil {
		responseError(w, err)
		return
	}

	w.Header().Add("Location", env.DashHost+"/dash#"+token)
	w.WriteHeader(http.StatusFound)
}

func handleRpList(w http.ResponseWriter, r *http.Request) {
	var body tokenRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	rps, err := listRp(r.Context())
	responseJsonData(w, r, rps, err)
}

type createRpRequest struct {
	ClientId string `json:"client_id"`
	tokenRequest
}

func handleRpCreate(w http.ResponseWriter, r *http.Request) {
	var body createRpRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	res, err := createRp(r.Context(), body.ClientId)
	responseJsonData(w, r, res, err)
}

func handleRpUpdateSecret(w http.ResponseWriter, r *http.Request) {
	var body createRpRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	res, err := changeRpSecret(r.Context(), body.ClientId)
	responseJsonData(w, r, res, err)
}

func handleRpDelete(w http.ResponseWriter, r *http.Request) {
	var body createRpRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	if err := deleteRp(r.Context(), body.ClientId); err != nil {
		responseJsonData(w, r, nil, err)
	}

	responseJsonData(w, r, nil, nil)
}

type addRpRedirectUriRequest struct {
	ClientId    string `json:"client_id"`
	RedirectUri string `json:"redirect_uri"`
	tokenRequest
}

func handleRpRedirecturiAdd(w http.ResponseWriter, r *http.Request) {
	var body addRpRedirectUriRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	if err := addRedirectUri(r.Context(), body.ClientId, body.RedirectUri); err != nil {
		responseJsonData(w, r, nil, err)
	}

	responseJsonData(w, r, nil, nil)
}

func handleRpRedirecturiRemove(w http.ResponseWriter, r *http.Request) {
	var body addRpRedirectUriRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	if err := removeRedirectUri(r.Context(), body.ClientId, body.RedirectUri); err != nil {
		responseJsonData(w, r, nil, err)
	}

	responseJsonData(w, r, nil, nil)
}

func handleUserList(w http.ResponseWriter, r *http.Request) {
	var body tokenRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	users, err := listUser(r.Context())
	responseJsonData(w, r, users, err)
}

type createUserRequest struct {
	UserId string `json:"user_id"`
	tokenRequest
}

func handleUserCreate(w http.ResponseWriter, r *http.Request) {
	var body createUserRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := createUser(r.Context(), body.UserId)
	responseJsonData(w, r, nil, err)
}

func handleUserDelete(w http.ResponseWriter, r *http.Request) {
	var body createUserRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := deleteUser(r.Context(), body.UserId)
	responseJsonData(w, r, nil, err)
}

type changeUserPasswordRequest struct {
	UserId   string `json:"user_id"`
	Password string `json:"password"`
	tokenRequest
}

func handleUserPasswordChange(w http.ResponseWriter, r *http.Request) {
	var body changeUserPasswordRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := changeUserPassword(r.Context(), body.UserId, body.Password)
	responseJsonData(w, r, nil, err)
}

func handleUserPasswordRemove(w http.ResponseWriter, r *http.Request) {
	var body createUserRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := deleteUserPassword(r.Context(), body.UserId)
	responseJsonData(w, r, nil, err)
}

func handleUserSessionList(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserSessionRevoke(w http.ResponseWriter, r *http.Request) {
	responseJsonData(w, r, nil, fmt.Errorf("unimplemented"))
}

func handleUserAuthenticationList(w http.ResponseWriter, r *http.Request) {
	var body createUserRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	data, err := listUserAuthentication(r.Context(), body.UserId)
	responseJsonData(w, r, data, err)
}

func handleListUserRole(w http.ResponseWriter, r *http.Request) {
	var body createUserRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	data, err := listUserRole(r.Context(), body.UserId)
	responseJsonData(w, r, data, err)
}

type setUserRoleRequest struct {
	Roles  []string `json:"roles"`
	UserId string   `json:"user_id"`
	tokenRequest
}

func handleSetUserRole(w http.ResponseWriter, r *http.Request) {
	var body setUserRoleRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := setUserRole(r.Context(), body.UserId, body.Roles)
	responseJsonData(w, r, nil, err)
}

func handleListRoleAccess(w http.ResponseWriter, r *http.Request) {
	var body createRpRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	data, err := listRoleAccess(r.Context(), body.ClientId)
	responseJsonData(w, r, data, err)
}

type setRoleAccessRequest struct {
	Roles    []string `json:"roles"`
	ClientId string   `json:"client_id"`
	tokenRequest
}

func handleSetRoleAccess(w http.ResponseWriter, r *http.Request) {
	var body setRoleAccessRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := setRoleAccess(r.Context(), body.ClientId, body.Roles)
	responseJsonData(w, r, nil, err)
}

type createRoleRequest struct {
	Name string `json:"name"`
	tokenRequest
}

func handleCreateRole(w http.ResponseWriter, r *http.Request) {
	var body createRoleRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := createRole(r.Context(), body.Name)
	responseJsonData(w, r, nil, err)
}

func handleDeleteRole(w http.ResponseWriter, r *http.Request) {
	var body createRoleRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	err := deleteRole(r.Context(), body.Name)
	responseJsonData(w, r, nil, err)
}

func handleListRole(w http.ResponseWriter, r *http.Request) {
	var body tokenRequest
	if !parseBody(w, r, &body) {
		return
	}

	if !authorizedOrReturn(r.Context(), w, body.Token) {
		return
	}

	data, err := listRole(r.Context())
	responseJsonData(w, r, data, err)
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

func authorizedOrReturn(ctx context.Context, w http.ResponseWriter, token string) (ok bool) {
	if !authorized(ctx, token) {
		w.WriteHeader(http.StatusUnauthorized)
		io.WriteString(w, `{ "message": "unauthorized" }`)
		return false
	}
	return true
}

func responseJsonData(w http.ResponseWriter, r *http.Request, data interface{}, err error) {
	if err != nil {
		responseError(w, err)
		return
	}

	if data == nil {
		fmt.Fprintln(w, "{}")
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

	res := errorResponse{
		Error: "error",
	}
	bytes, err := json.Marshal(res)
	if err != nil {
		// 通常起こりえないので panic
		panic(err)
	}

	// TODO: エラーレスポンスを実装する
	w.WriteHeader(http.StatusBadRequest)
	io.WriteString(w, string(bytes))
}
