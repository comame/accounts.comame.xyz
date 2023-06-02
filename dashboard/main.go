package main

import (
	"embed"
	"log"

	"github.com/comame/readenv-go"
	router "github.com/comame/router-go"
)

type env_t struct {
	Host         string `env:"HOST"`
	DashHost     string `env:"DASH_HOST"`
	ClientSecret string `env:"CLIENT_SECRET"`
}

var env env_t

//go:embed web/dist/*
var static embed.FS

func init() {
	readenv.Read(&env)
	log.SetPrefix("[ADMIN] ")
	log.SetFlags(log.Lshortfile | log.LstdFlags)
}

func main() {
	router.Get("/", handleStatic)
	router.Get("/signin", handleSignin)
	router.Get("/callback", handleCallback)

	router.Post("/rp/list", handleRpList)
	router.Post("/rp/create", handleRpCreate)
	router.Post("/rp/update_secret", handleRpUpdateSecret)
	router.Post("/rp/delete", handleRpDelete)
	router.Post("/rp/redirect_uri/add", handleRpRedirecturiAdd)
	router.Post("/rp/redirect_uri/remove", handleRpRedirecturiRemove)
	router.Post("/rp/role/list", handleListRoleAccess)
	router.Post("/rp/role/set", handleSetRoleAccess)

	router.Post("/user/list", handleUserList)
	router.Post("/user/create", handleUserCreate)
	router.Post("/user/delete", handleUserDelete)
	router.Post("/user/password/change", handleUserPasswordChange)
	router.Post("/user/password/remove", handleUserPasswordRemove)
	router.Post("/user/session/list", handleUserSessionList)
	router.Post("/user/session/revoke", handleUserSessionRevoke)
	router.Post("/user/authentication/list", handleUserAuthenticationList)
	router.Post("/user/role/list", handleListUserRole)
	router.Post("/user/role/set", handleSetUserRole)
	router.Post("/user/userinfo/get", handleGetUserinfo)

	router.Post("/role/list", handleListRole)
	router.Post("/role/create", handleCreateRole)
	router.Post("/role/delete", handleDeleteRole)

	router.All("/*", handleStatic)

	log.Println("Start http://localhost:8081")

	router.ListenAndServe(":8081")
}
