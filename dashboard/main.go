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
	router.Get("/dash", handleStatic)
	router.Get("/dash/signin", handleSignin)
	router.Get("/dash/callback", handleCallback)

	router.Post("/dash/rp/list", handleRpList)
	router.Post("/dash/rp/create", handleRpCreate)
	router.Post("/dash/rp/update_secret", handleRpUpdateSecret)
	router.Post("/dash/rp/delete", handleRpDelete)
	router.Post("/dash/rp/redirect_uri/add", handleRpRedirecturiAdd)
	router.Post("/dash/rp/redirect_uri/remove", handleRpRedirecturiRemove)

	router.Post("/dash/user/list", handleUserList)
	router.Post("/dash/user/create", handleUserCreate)
	router.Post("/dash/user/delete", handleUserDelete)
	router.Post("/dash/user/password/change", handleUserPasswordChange)
	router.Post("/dash/user/password/remove", handleUserPasswordRemove)
	router.Post("/dash/user/session/list", handleUserSessionList)
	router.Post("/dash/user/session/revoke", handleUserSessionRevoke)
	router.Post("/dash/user/authentication/list", handleUserAuthenticationList)

	router.All("/*", handleStatic)

	log.Println("Start http://localhost:8081")

	router.ListenAndServe(":8081")
}
