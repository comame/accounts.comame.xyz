package main

import (
	"io"
	"net/http"
)

func handleIndex(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleSignin(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleCallback(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpList(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpCreate(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpUpdatesecret(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpDelete(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpRedirecturiAdd(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleRpRedirecturiRemove(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserList(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserCreate(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserDelete(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserPasswordChange(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserPasswordRemove(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserSessionList(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserSessionRevoke(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleUserAuthenticationList(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "unimplemented\n")
}

func handleNotFound(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusNotFound)
	io.WriteString(w, "not found\n")
}
