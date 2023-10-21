package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"net/url"
	"os"

	"github.com/comame/accounts.comame.xyz/auth"
	"github.com/comame/accounts.comame.xyz/kvs"
	"github.com/comame/accounts.comame.xyz/oidc"
	"github.com/comame/mysql-go"
	"github.com/comame/router-go"
)

func init() {
	if err := mysql.Initialize(); err != nil {
		panic(err)
	}

	// TODO: 環境変数から読む
	kvs.Initialize("dev.accounts.comame.xyz", "redis.comame.dev:6379")
}

func main() {
	// TODO: いずれ消す
	tmpNotFound := func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusNotFound)
		io.WriteString(w, "unimplemented")
	}

	router.Get("/signin", handle_GET_signin)

	router.Get("/authenticate", handle_GET_authenticate)
	router.Post("/authenticate", handle_POST_authenticate)
	router.Post("/code", tmpNotFound)
	router.Get("/userinfo", tmpNotFound)
	router.Post("/userinfo", tmpNotFound)
	router.Get("/.well-known/openid-configuration", tmpNotFound)
	router.Get("/certs", tmpNotFound)

	router.Post("/signin/google", tmpNotFound)
	router.Post("/api/signin-password", handle_GET_apiSigninPassword)
	router.Get("/oidc-callback/google", tmpNotFound)

	router.Get("/*", handle_GET_rest)
	router.All("/*", tmpNotFound)

	log.Println("Start http://localhost:8080")
	http.ListenAndServe(":8080", router.Handler())
}

func handle_GET_signin(w http.ResponseWriter, r *http.Request) {
	q := r.URL.Query()

	stateID := q.Get("sid")
	clientID := q.Get("cid")

	if stateID == "" || clientID == "" {
		// TODO: ちゃんとエラー画面を出す
		io.WriteString(w, "err")
		return
	}

	// TODO: 静的に読みたい
	f, err := os.Open("/home/comame/github.com/comame/id/static/front/src/signin.html")
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		io.WriteString(w, "error")
		return
	}
	defer f.Close()

	io.Copy(w, f)
}

func handle_GET_authenticate(w http.ResponseWriter, r *http.Request) {
	authenticationRequest(w, r.URL.Query())
}

func handle_POST_authenticate(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseForm(); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "bad_request" }`)
		return
	}

	authenticationRequest(w, r.Form)
}

type req_GET_apiSigninPassword struct {
	UserId         string `json:"user_id"`
	Password       string `json:"password"`
	CSRFToken      string `json:"csrf_token"`
	RelyingPartyID string `json:"relying_party_id"`
	UserAgentID    string `json:"user_agent_id"`
	StateID        string `json:"state_id"`
}

func handle_GET_apiSigninPassword(w http.ResponseWriter, r *http.Request) {
	b, err := io.ReadAll(r.Body)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `"{"error": "bad_request" }`)
		return
	}

	var req req_GET_apiSigninPassword
	if err := json.Unmarshal(b, &req); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `"{"error": "bad_request" }`)
		return
	}

	passOk, err := auth.AuthenticateByPassword(r.Context(), req.UserId, req.Password, req.RelyingPartyID, req.UserAgentID)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "bad_request" }`)
		return
	}
	if !passOk {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "invalid_credential" }`)
		return
	}

	roleOk, err := auth.Authorized(req.UserId, req.RelyingPartyID)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "bad_request" }`)
		return
	}
	if !roleOk {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "unauthorized" }`)
	}

	// TODO: post_authentication
}

func handle_GET_rest(w http.ResponseWriter, r *http.Request) {
	// TODO: 静的に読むようにしたい
	srv := http.FileServer(http.Dir("/home/comame/github.com/comame/id/static"))
	srv.ServeHTTP(w, r)
}

func authenticationRequest(w http.ResponseWriter, body url.Values) {
	req, err := oidc.ParseAuthenticationRequestFromQuery(body)
	if err != nil {
		log.Println("failed to parse authenticationRequest")
		log.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "bad_request" }`)
		return
	}

	if err := req.Assert(); err != nil {
		log.Println("failed to assert authenticationRequest")
		log.Println(err)
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, `{ "error": "bad_request" }`)
		return
	}

	id, err := oidc.PreAuthenticate(*req)
	if err != nil {
		log.Println(err)
		perr, ok := err.(oidc.PreAuthenticateError)
		if !ok {
			w.WriteHeader(http.StatusBadRequest)
			io.WriteString(w, `{ "error": "bad_request" }`)
			return
		}
		if !perr.NotifyToClient {
			w.WriteHeader(http.StatusBadRequest)
			io.WriteString(w, `{ "error": "bad_request" }`)
			return
		}

		u, err := url.Parse(req.RedirectURI)
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			io.WriteString(w, `{ "error": "bad_request" }`)
			return
		}

		q := u.Query()
		q.Add("error", perr.Error())
		if req.State != "" {
			q.Add("state", req.State)
		}
		u.RawQuery = q.Encode()

		w.Header().Add("Location", u.String())
		w.WriteHeader(http.StatusFound)
		return
	}

	u := fmt.Sprintf("/signin?sid=%s&cid=%s", id, req.ClientId)
	w.Header().Add("Location", u)
	w.WriteHeader(http.StatusFound)
}
