package ceremony

import (
	"fmt"
	"io"
	"net/http"
	"net/url"
	"slices"

	"github.com/comame/accounts.comame.xyz/internal/db"
	"github.com/comame/accounts.comame.xyz/internal/kvs"
	"github.com/comame/accounts.comame.xyz/internal/oidc"
)

const badRequestJSONString = `{ "error": "bad_request" }`

func AuthenticationRequest(w http.ResponseWriter, body url.Values) {
	request, err := oidc.ParseAuthenticationRequest(body)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		io.WriteString(w, badRequestJSONString)
		return
	}

	// FIXME: redirect_uri が信頼できないのにリダイレクトでエラーを返している
	// request パラメータは非対応
	if request.Request != "" {
		redirectErrorResponse(w, request.RedirectURI, request.State, oidc.ErrAuthenticationErrRequestNotSupported)
		return
	}

	// prompt パラメータは非対応
	if request.Prompt != oidc.LoginPromptUnspecified {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// max_age パラメータは非対応
	if request.MaxAge >= 0 {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// 必須パラメータのチェック
	if request.Scope == "" || request.ResponseType == "" || request.ClientId == "" || request.RedirectURI == "" {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// Relying Party の存在確認
	if _, err := db.RelyingParty_select(request.ClientId); err != nil {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// redirect_uri の完全一致確認
	redirectURIs, err := db.RelyingParty_selectRedirectURIs(request.ClientId)
	if err != nil {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}
	if !slices.Contains(redirectURIs, request.RedirectURI) {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// scope に openid が含まれるか確認
	if !oidc.ContainsScope(request.Scope, "openid") {
		redirectErrorResponse(w, request.RedirectURI, request.State, oidc.ErrAuthenticationErrInvalidScope)
		return
	}

	flow := oidc.IdentFlowFromResponseType(request.ResponseType)
	if flow == oidc.FlowUnused {
		displayErrorResponse(w, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	// Implicit Flow では nonce が必須
	if flow == oidc.FlowImplicit && request.Nonce == "" {
		redirectErrorResponse(w, request.RedirectURI, request.State, oidc.ErrAuthenticationErrInvalidRequest)
		return
	}

	sessionID, err := kvs.LoginSession_save(request.ClientId, request.RedirectURI, request.Scope, request.State, request.Nonce, string(flow))
	if err != nil {
		displayErrorResponse(w, oidc.ErrAuthenticationErrServerError)
		return
	}

	continueToSignInPage(w, sessionID, request.ClientId)
}

func displayErrorResponse(w http.ResponseWriter, _ oidc.AuthenticationError) {
	w.WriteHeader(http.StatusBadRequest)
	io.WriteString(w, badRequestJSONString)
}

func redirectErrorResponse(w http.ResponseWriter, redirectURI, state string, message oidc.AuthenticationError) {
	u, err := url.Parse(redirectURI)
	if err != nil {
		displayErrorResponse(w, message)
		return
	}

	q := u.Query()
	q.Add("error", string(message))
	if state != "" {
		q.Add("state", state)
	}
	u.RawQuery = q.Encode()

	w.Header().Add("Location", u.String())
	w.WriteHeader(http.StatusFound)
}

func continueToSignInPage(w http.ResponseWriter, sessionID, clientID string) {
	u := fmt.Sprintf("/signin?sid=%s&cid=%s", sessionID, clientID)
	w.Header().Add("Location", u)
	w.WriteHeader(http.StatusFound)
}
