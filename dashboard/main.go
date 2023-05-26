package main

import (
	"io"
	"log"
	"net/http"

	router "github.com/comame/router-go"
)

func main() {
	router.Get("/", func(w http.ResponseWriter, r *http.Request) {
		io.WriteString(w, "hello")
	})

	log.Println("Start http://localhost:8081")

	router.ListenAndServe(":8081")
}
