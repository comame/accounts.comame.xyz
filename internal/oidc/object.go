package oidc

type AuthenticationRequest struct {
	Scope        string
	ResponseType string
	ClientId     string
	RedirectURI  string
	State        string
	Nonce        string
	Prompt       LoginPrompt
	// Negative MaxAge (-1) indicates unspecified.
	MaxAge      int64
	IDTokenHint string
	// unsupported parameter
	Request string
}

type LoginPrompt string

var (
	LoginPromptUnspecified   LoginPrompt = ""
	LoginPromptNone          LoginPrompt = "none"
	LoginPromptLogin         LoginPrompt = "login"
	LoginPromptConsent       LoginPrompt = "consent"
	LoginPromptSelectAccount LoginPrompt = "select_account"
)

type AuthenticationResponse struct {
	State   string
	Code    string
	IDToken string
	Error   string

	Flow        Flow
	RedirectURI string
}

type codeRequest struct {
	ClientID     string
	ClientSecret string
	GrantType    string
	Code         string
	RedirectURI  string
}

type codeResponse struct {
	AccessToken  string `json:"access_token,omitempty"`
	TokenType    string `json:"token_type,omitempty"`
	ExpiresIn    int64  `json:"expires_in,omitempty"`
	RefreshToken string `json:"refresh_token,omitempty"`
	Scope        string `json:"scope,omitempty"`
	Error        string `json:"error,omitempty"`
	IDToken      string `json:"id_token,omitempty"`
}

type discovery struct {
	Issuer                            string   `json:"issuer"`
	AuthorizationEndpoint             string   `json:"authorization_endpoint"`
	TokenEndpoint                     string   `json:"token_endpoint"`
	UserInfoEndpoint                  string   `json:"userinfo_endpoint"`
	JWKsURI                           string   `json:"jwks_uri"`
	ResponseTypesSupported            []string `json:"response_types_supported"`
	SubjectTypesSupported             []string `json:"subject_types_supported"`
	IDTokenSigningAlgValuesSupported  []string `json:"id_token_signing_alg_values_supported"`
	ScopesSupported                   []string `json:"scopes_supported"`
	TokenEndpointAuthMethodsSupported []string `json:"token_endpoint_auth_methods_supported"`
	ClaimsSupported                   []string `json:"claims_supported"`
	CodeChallengeMethodsSupported     []string `json:"code_challenge_methods_supported"`
	GrantTypesSupported               []string `json:"grant_types_supported"`
}

type userInfo struct {
	Sub               string `json:"sub"`
	Email             string `json:"email"`
	EmailVerified     bool   `json:"email_verified"`
	Name              string `json:"name"`
	PreferredUsername string `json:"preferred_username"`
	Profile           string `json:"profile"`
	Picture           string `json:"picture"`
}
